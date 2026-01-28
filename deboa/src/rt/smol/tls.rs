#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use std::sync::Arc;

#[cfg(any(feature = "http1", feature = "http2"))]
use crate::rt::smol::stream::SmolStream;
#[cfg(any(feature = "http1", feature = "http2"))]
use smol::net::TcpStream;

#[cfg(feature = "smol-native-tls")]
use async_native_tls::{Certificate, Identity, TlsConnector};

#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use crate::client::conn::stream::setup_rust_tls;
#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use futures_rustls::TlsConnector;
#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use rustls::pki_types::ServerName;

#[cfg(all(
    any(feature = "smol-rust-tls", feature = "smol-native-tls"),
    any(feature = "http1", feature = "http2")
))]
use crate::{
    cert::{Certificate as DeboaCertificate, Identity as DeboaIdentity},
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(any(feature = "http1", feature = "http2"))]
async fn create_stream(host: &str, port: u16) -> Result<TcpStream> {
    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: format!("Could not connect to server: {}", e),
        }));
    }

    Ok(stream.unwrap())
}

#[cfg(any(feature = "http1", feature = "http2"))]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<SmolStream> {
    let stream = create_stream(host, port).await?;
    Ok(SmolStream::Plain(stream))
}

#[cfg(all(any(feature = "http1", feature = "http2"), feature = "smol-native-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: &[&str],
) -> Result<SmolStream> {
    let socket = create_stream(host, port).await?;
    let builder = TlsConnector::new();

    let builder = if skip_server_verification {
        builder
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
    } else {
        builder
    };

    let builder = builder.request_alpns(&alpn);

    let builder = if let Some(ca) = certificate {
        let cert: std::result::Result<Certificate, std::io::Error> = ca.try_into();
        if let Err(e) = cert {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: format!("Invalid CA certificate: {}", e),
            }));
        }

        builder.add_root_certificate(cert.unwrap())
    } else {
        builder
    };

    let builder = if let Some(identity) = identity {
        let ident: std::result::Result<Identity, std::io::Error> = identity.try_into();
        if let Err(e) = ident {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: format!("Invalid client identity: {}", e),
            }));
        }
        builder.identity(ident.unwrap())
    } else {
        builder
    };

    let stream = builder
        .connect(host.to_string(), socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: format!("Could not connect to server: {}", e),
        }));
    }

    let stream = stream.unwrap();
    Ok(SmolStream::Tls(stream))
}

#[cfg(all(any(feature = "http1", feature = "http2"), feature = "smol-rust-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<SmolStream> {
    let socket = create_stream(host, port).await?;
    let config = setup_rust_tls(host, identity, certificate, skip_server_verification, alpn)?;
    let connector = TlsConnector::from(Arc::new(config));
    let hostname = ServerName::try_from(host.to_string());

    if let Err(e) = hostname {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = connector
        .connect(hostname.unwrap(), socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: format!("Could not connect to server: {}", e),
        }));
    }

    let stream = stream.unwrap();
    Ok(SmolStream::Tls(Box::new(stream)))
}
