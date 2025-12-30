use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use futures::future;
use h3::client::{Connection, SendRequest};
use http::version::Version;
use hyper::{Request, Response};
use quinn::crypto::rustls::QuicClientConfig;
use quinn::Endpoint;

use crate::request::Http3Request;
use crate::{
    cert::ClientCert,
    client::conn::{udp::DeboaUdpConnection, BaseHttpConnection},
    errors::{ConnectionError, DeboaError},
    Result,
};

async fn lookup_and_connect(
    host: &str,
    port: u16,
    client_endpoint: Endpoint,
) -> std::result::Result<
    (Connection<h3_quinn::Connection, Bytes>, SendRequest<h3_quinn::OpenStreams, Bytes>),
    Box<dyn std::error::Error>,
> {
    let addr = tokio::net::lookup_host((host, port))
        .await?
        .next()
        .unwrap();

    let conn = client_endpoint
        .connect(addr, host)?
        .await?;

    let quinn_conn: h3_quinn::Connection = h3_quinn::Connection::new(conn);

    let http_conn = h3::client::new(quinn_conn).await?;

    Ok(http_conn)
}

#[async_trait]
impl DeboaUdpConnection for BaseHttpConnection<Http3Request> {
    type Sender = Http3Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect(
        is_secure: bool,
        host: &str,
        port: u16,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Http3Request>> {
        let mut client_endpoint =
            Endpoint::client(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0)));

        if let Err(e) = client_endpoint {
            return Err(DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }));
        }

        let mut client_endpoint = client_endpoint.unwrap();

        if is_secure {
            let root_store =
                rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
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
        }

        let result = lookup_and_connect(host, port, client_endpoint).await;

        if let Err(e) = result {
            return Err(DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }));
        }

        let (mut conn, sender) = result.unwrap();

        tokio::spawn(async move {
            Err::<(), h3::error::ConnectionError>(future::poll_fn(|cx| conn.poll_close(cx)).await)
        });

        Ok(BaseHttpConnection::<Http3Request> { sender })
    }

    async fn send_request(&mut self, request: Request<()>) -> Result<Response<Bytes>> {
        let method = request
            .method()
            .to_string();
        let result = self
            .sender
            .send_request(request)
            .await;

        self.process_response(&method, result)
            .await
    }
}

impl crate::client::conn::udp::private::DeboaUdpConnectionSealed
    for BaseHttpConnection<Http3Request>
{
}
