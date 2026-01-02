use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures::future;
use http::{version::Version, StatusCode};
use http_body_util::Full;
use hyper::{Request, Response};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::Endpoint;

use crate::request::Http3Request;
use crate::{
    cert::ClientCert,
    client::conn::{udp::DeboaUdpConnection, BaseHttpConnection},
    errors::{ConnectionError, DeboaError, RequestError, ResponseError},
    Result,
};

async fn lookup_and_connect(
    host: &str,
    port: u16,
    client_endpoint: &Endpoint,
) -> std::result::Result<h3_quinn::Connection, Box<dyn std::error::Error>> {
    let addr = tokio::net::lookup_host((host, port))
        .await?
        .next()
        .unwrap();

    let conn = client_endpoint
        .connect(addr, host)?
        .await?;

    let quinn_conn: h3_quinn::Connection = h3_quinn::Connection::new(conn);

    Ok(quinn_conn)
}

#[async_trait]
impl DeboaUdpConnection for BaseHttpConnection<Http3Request> {
    type Sender = Http3Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect(
        host: &str,
        port: u16,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Http3Request>> {
        let client_endpoint =
            Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0)));

        if let Err(e) = client_endpoint {
            return Err(DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }));
        }

        let mut client_endpoint = client_endpoint.unwrap();

        let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let mut tls_config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("Failed to set TLS version")
            .with_root_certificates(root_store)
            .with_no_client_auth();

        tls_config.enable_early_data = true;
        tls_config.alpn_protocols = vec![b"h3".to_vec()];

        let quic_config = QuicClientConfig::try_from(tls_config);
        if let Err(e) = quic_config {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: e.to_string(),
            }));
        }

        let quic_config = quic_config.unwrap();

        let client_config = quinn::ClientConfig::new(Arc::new(quic_config));
        client_endpoint.set_default_client_config(client_config);

        let result = lookup_and_connect(host, port, &client_endpoint).await;

        if let Err(e) = result {
            return Err(DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }));
        }

        let conn = result.unwrap();

        let client = h3::client::new(conn).await;

        if let Err(e) = client {
            return Err(DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }));
        }

        let (mut conn, send_request) = client.unwrap();

        tokio::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        Ok(BaseHttpConnection::<Http3Request> { sender: send_request })
    }

    async fn send_request(&mut self, request: Request<()>) -> Result<Response<Full<Bytes>>> {
        let mut sender = self.sender.clone();

        let method = request
            .method()
            .to_string();

        let request = sender
            .send_request(request)
            .await;

        if let Err(err) = request {
            return Err(DeboaError::Request(RequestError::Send {
                url: "".to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let request_stream = request.unwrap();
        let (mut send_stream, mut recv_stream) = request_stream.split();

        if method == "POST" || method == "PUT" || method == "PATCH" {
            // Send request body if present
            // For now, we're not handling the body, but we could add that later
            let buf = Bytes::from("dummy body"); // Placeholder - in reality you'd use the actual request body
            send_stream
                .send_data(buf)
                .await;
        }

        let finish_request = send_stream
            .finish()
            .await;
        if let Err(err) = finish_request {
            return Err(DeboaError::Request(RequestError::Send {
                url: "".to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
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

        let response = self
            .process_response(response, recv_stream)
            .await?;

        Ok(response)
    }
}

impl crate::client::conn::udp::private::DeboaUdpConnectionSealed
    for BaseHttpConnection<Http3Request>
{
}
