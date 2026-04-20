use std::{marker::PhantomData, net::SocketAddr, sync::Arc};

use deboa::{
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    request::Http3Request,
};
use futures::future;
use http::{version::Version, StatusCode};
use http_body_util::BodyExt;
use hyper::{Request, Response};
use hyper_body_utils::HttpBody;
use quinn::{crypto::rustls::QuicClientConfig, Endpoint};
use trust_dns_resolver::error::ResolveErrorKind;

use crate::{
    alpn,
    client::conn::{
        rustls::setup_rust_tls, udp::DeboaUdpConnection, BaseHttpConnection, ConnectionConfig,
    },
    Result,
};

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

async fn lookup_and_connect(
    host: &str,
    port: u16,
    client_endpoint: &Endpoint,
) -> std::result::Result<h3_quinn::Connection, DeboaError> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    let response = resolver
        .lookup_ip(host)
        .await;

    let addr = match response {
        Ok(response) => response,
        Err(e) => match e.kind() {
            ResolveErrorKind::NoRecordsFound { query, .. } => {
                let query_name = query
                    .name()
                    .to_string();
                return Err(DeboaError::Connection(ConnectionError::Udp {
                    host: host.to_string(),
                    message: format!("Could not resolve host: {}", query_name),
                }));
            }
            _ => {
                return Err(DeboaError::Connection(ConnectionError::Udp {
                    host: host.to_string(),
                    message: format!("Could not resolve host: {}", e),
                }));
            }
        },
    };

    let addr = addr
        .iter()
        .next()
        .expect("no addresses returned!");

    let conn = client_endpoint.connect(SocketAddr::new(addr, port), host);

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

impl DeboaUdpConnection for BaseHttpConnection<Http3Request, HttpBody, HttpBody> {
    type Sender = Http3Request;
    type ReqBody = HttpBody;
    type ResBody = HttpBody;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a>,
    ) -> Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>> {
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

        let result = lookup_and_connect(config.host(), config.port(), &client_endpoint).await;

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

        let (mut conn, send_request) = client.unwrap();

        tokio::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(BaseHttpConnection::<Self::Sender, Self::ReqBody, Self::ResBody> {
            sender: send_request,
            req_body: PhantomData,
            res_body: PhantomData,
        })
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

impl crate::client::conn::udp::private::DeboaUdpConnectionSealed
    for BaseHttpConnection<Http3Request, HttpBody, HttpBody>
{
}
