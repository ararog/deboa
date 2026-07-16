use crate::{
    alpn,
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{stream::setup_rust_tls, BaseHttpConnection, Http3Connection},
    Result,
};
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError},
};
use deboa_h3::generic::{Http3Request, SendRequest};
use futures::future;
use http::version::Version;
use hyper_body_utils::HttpBody;
use quinn::{crypto::rustls::QuicClientConfig, Endpoint};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
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

impl HttpConnection for Http3Connection {
    type Sender = Http3Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http3Connection {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = Http3Connection;
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

        smol::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .detach();

        Ok(BaseHttpConnection::new(SendRequest::new(sender)))
    }
}
