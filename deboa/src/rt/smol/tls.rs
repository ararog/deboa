#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use std::sync::Arc;

#[cfg(any(feature = "http1", feature = "http2"))]
use crate::rt::smol::stream::SmolStream;
#[cfg(any(feature = "http1", feature = "http2"))]
use smol::net::TcpStream;

#[cfg(feature = "smol-native-tls")]
use async_native_tls::{Certificate, Identity, TlsConnector};

#[cfg(feature = "smol-rust-tls")]
use crate::client::conn::stream::setup_rust_tls;
#[cfg(all(feature = "smol-rust-tls", any(feature = "http1", feature = "http2")))]
use futures_rustls::TlsConnector;
#[cfg(feature = "smol-rust-tls")]
use rustls::pki_types::ServerName;

use crate::{
    cert::Identity as DeboaIdentity,
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(any(feature = "http1", feature = "http2"))]
async fn create_stream(host: &str, port: u16) -> Result<TcpStream> {
    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    Ok(stream.unwrap())
}

#[cfg(all(feature = "http1", feature = "http2"))]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<SmolStream> {
    let stream = create_stream(host, port).await?;
    Ok(SmolStream::Plain(stream))
}

#[cfg(all(any(feature = "http1", feature = "http2"), feature = "smol-native-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<DeboaIdentity>,
    skip_server_verification: bool,
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

    let builder = if let Some(client_cert) = client_cert {
        let builder = if let Some(ca) = client_cert.ca() {
            let pem = std::fs::read(ca);
            if let Err(e) = pem {
                return Err(DeboaError::ClientCert { message: e.to_string() });
            }
            let cert = Certificate::from_pem(&pem.unwrap());
            builder.add_root_certificate(cert.unwrap())
        } else {
            builder
        };

        let file = std::fs::read(client_cert.cert());
        if let Err(e) = file {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }
        let identity = Identity::from_pkcs12(
            &file.unwrap(),
            client_cert
                .pw()
                .unwrap_or_default(),
        );
        if let Err(e) = identity {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }

        builder.identity(identity.unwrap())
    } else {
        builder
    };

    let stream = builder
        .connect(host.to_string(), socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = stream.unwrap();
    Ok(SmolStream::Tls(stream))
}

#[cfg(all(any(feature = "http1", feature = "http2"), feature = "smol-rust-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<DeboaIdentity>,
    skip_server_verification: bool,
) -> Result<SmolStream> {
    let socket = create_stream(host, port).await?;
    let config = setup_rust_tls(host, client_cert, skip_server_verification)?;
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
            message: e.to_string(),
        }));
    }

    let stream = stream.unwrap();
    Ok(SmolStream::Tls(Box::new(stream)))
}
