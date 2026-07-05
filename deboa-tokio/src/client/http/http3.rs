use crate::{
    MAX_ERROR_MESSAGE_SIZE, alpn, cert::{DeboaCertificate, DeboaIdentity}, client::http::conn::{BaseHttpConnection, stream::tls::setup_rust_tls},
};
use bytes::{Buf, Bytes};
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    request::Http3Request,
    Result,
};
use futures::future;
use h3::client::RequestStream;
use h3_quinn::RecvStream;
use http::{StatusCode, response::Parts, version::Version};
use http_body_util::BodyExt;
use hyper::{Request, Response};
use hyper_body_utils::HttpBody;
use quinn::{crypto::rustls::QuicClientConfig, Endpoint};
use std::{
    future::Future, net::{IpAddr, SocketAddr}, sync::Arc,
};

async fn lookup_and_connect(
    ip: IpAddr,
    host: &str,
    port: u16,
    client_endpoint: &Endpoint,
) -> std::result::Result<h3_quinn::Connection, DeboaError> {
    let conn = client_endpoint.connect(SocketAddr::new(ip, port), host);

    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: host.to_string(),
                message: format!("Could not connect to server: {}", e),
            }))
        }
    };

    let conn = conn.await;

    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => match e {
            quinn::ConnectionError::TransportError(e) => {
                return Err(DeboaError::Connection(ConnectionError::Tls {
                    host: host.to_string(),
                    message: format!("Could not connect to server: {}", e),
                }))
            }
            _ => {
                return Err(DeboaError::Connection(ConnectionError::Udp {
                    host: host.to_string(),
                    message: format!("Could not connect to server: {}", e),
                }))
            }
        },
    };

    let quinn_conn: h3_quinn::Connection = h3_quinn::Connection::new(conn);

    Ok(quinn_conn)
}

impl BaseHttpConnection<Http3Request, HttpBody, HttpBody> {
    fn process_response(
        &self,
        parts: Parts,
        mut stream: RequestStream<RecvStream, Bytes>,
    ) -> impl Future<Output = Result<Response<HttpBody>>> + Send {
        async move {
            let status_code = parts.status;

            if (!status_code.is_success()
                && !status_code.is_informational()
                && !status_code.is_redirection())
                || status_code == StatusCode::TOO_MANY_REQUESTS
            {
                let mut error_message = Vec::new();
                let mut downloaded = 0;
                while let Ok(Some(chunk)) = stream
                    .recv_data()
                    .await
                {
                    if downloaded + error_message.len() > MAX_ERROR_MESSAGE_SIZE {
                        break;
                    }
                    error_message.extend_from_slice(chunk.chunk());
                    downloaded += error_message.len();
                }

                return Err(DeboaError::Response(ResponseError::Receive {
                    status_code,
                    message: format!(
                        "Could not process request ({}): {}",
                        status_code,
                        String::from_utf8_lossy(&error_message)
                    ),
                }));
            }

            let body = HttpBody::from_quic_client(stream);
            let response = Response::from_parts(parts, body);

            Ok(response)
        }
    }
}

impl HttpConnection for BaseHttpConnection<Http3Request, HttpBody, HttpBody> {
    type Sender = Http3Request;
    type ReqBody = HttpBody;
    type ResBody = HttpBody;

    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for BaseHttpConnection<Http3Request, HttpBody, HttpBody> {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = BaseHttpConnection<Http3Request, HttpBody, HttpBody>;
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<Self::Connection> {
        let client_endpoint = Endpoint::client(SocketAddr::new(*config.client_bind_addr(), 0));

        if let Err(e) = client_endpoint {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let mut client_endpoint = client_endpoint.unwrap();

        let tls_config = setup_rust_tls(
            config.host(),
            config.identity(),
            config.certificate(),
            config.skip_cert_verification(),
            alpn(),
        )?;

        let quic_config = QuicClientConfig::try_from(tls_config);
        if let Err(e) = quic_config {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let quic_config = quic_config.unwrap();

        let client_config = quinn::ClientConfig::new(Arc::new(quic_config));
        client_endpoint.set_default_client_config(client_config);

        let result =
            lookup_and_connect(*config.ip(), config.host(), config.port(), &client_endpoint).await;

        if let Err(e) = result {
            return Err(e);
        }

        let conn = result.unwrap();

        let client = h3::client::new(conn).await;

        if let Err(e) = client {
            return Err(DeboaError::Connection(ConnectionError::Udp {
                host: config
                    .host()
                    .to_string(),
                message: e.to_string(),
            }));
        }

        let (mut conn, sender) = client.unwrap();

        tokio::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(BaseHttpConnection::new(sender))
    }

    async fn send_request(
        &mut self,
        request: Request<Self::ReqBody>,
    ) -> Result<Response<Self::ResBody>> {
        let mut sender = self.sender.clone();

        let url = request
            .uri()
            .to_string();

        let method = request
            .method()
            .to_string();

        let (parts, mut body) = request.into_parts();

        let bodyless_request = Request::from_parts(parts, ());

        let request = sender
            .send_request(bodyless_request)
            .await;

        if let Err(err) = request {
            return Err(DeboaError::Request(RequestError::Send { message: err.to_string() }));
        }

        let request_stream = request.unwrap();
        let (mut send_stream, mut recv_stream) = request_stream.split();

        if method == "POST" || method == "PUT" || method == "PATCH" {
            while let Some(chunk) = body.frame().await {
                let frame = chunk.unwrap();
                if let Some(bytes) = frame.data_ref() {
                    let result = send_stream
                        .send_data(bytes.clone())
                        .await;

                    if let Err(err) = result {
                        return Err(DeboaError::Request(RequestError::Send {
                            message: err.to_string(),
                        }));
                    }
                }
            }
        }

        let finish_request = send_stream
            .finish()
            .await;
        if let Err(err) = finish_request {
            return Err(DeboaError::Request(RequestError::Send { message: err.to_string() }));
        }

        let response = recv_stream
            .recv_response()
            .await;
        if let Err(err) = response {
            return Err(DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.to_string(),
            }));
        }

        let (parts, _) = response
            .unwrap()
            .into_parts();

        let response = self
            .process_response(parts, recv_stream)
            .await?;

        Ok(response)
    }
}
