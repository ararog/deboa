use std::sync::Arc;

#[cfg(feature = "smol-rt")]
use crate::rt::smol::stream::SmolStream;
#[cfg(feature = "tokio-rt")]
use crate::rt::tokio::stream::TokioStream;
#[cfg(feature = "smol-rt")]
use smol::net::TcpStream;
#[cfg(feature = "tokio-rt")]
use tokio::net::TcpStream;

#[cfg(feature = "tokio-native-tls")]
use tokio_native_tls::native_tls::{Certificate, Identity, TlsConnector};
#[cfg(feature = "tokio-rust-tls")]
use tokio_rustls::TlsConnector;

use crate::{
    cert::ClientCert,
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(feature = "tokio-rt")]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<TokioStream> {
    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    Ok(TokioStream::Plain(stream.unwrap()))
}

#[cfg(feature = "smol-rt")]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<SmolStream> {
    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    Ok(SmolStream::Plain(stream.unwrap()))
}

#[cfg(all(feature = "tokio-rt", feature = "tokio-native-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    client_cert: &Option<ClientCert>,
) -> Result<TokioStream> {
    let stream = { TcpStream::connect(host).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let socket = stream.unwrap();
    let mut builder = TlsConnector::builder();
    if let Some(client_cert) = client_cert {
        let file = std::fs::read(client_cert.cert());
        if let Err(e) = file {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }
        let identity = Identity::from_pkcs12(&file.unwrap(), client_cert.pw());
        if let Err(e) = identity {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }
        builder.identity(identity.unwrap());

        if let Some(ca) = client_cert.ca() {
            let pem = std::fs::read(ca);
            if let Err(e) = pem {
                return Err(DeboaError::ClientCert { message: e.to_string() });
            }
            let cert = Certificate::from_pem(&pem.unwrap());
            builder.add_root_certificate(cert.unwrap());
        }
    }

    let connector = builder
        .build()
        .unwrap();
    let connector = tokio_native_tls::TlsConnector::from(connector);
    let stream = connector
        .connect(host, socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = stream.unwrap();
    Ok(TokioStream::Tls(stream))
}

#[cfg(all(feature = "tokio-rt", feature = "tokio-rust-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<ClientCert>,
) -> Result<TokioStream> {
    use rustls::pki_types::ServerName;

    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let socket = stream.unwrap();
    let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS12])
        .expect("Failed to set TLS version")
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let parsed_hostname = ServerName::try_from(host.to_string());

    if let Err(e) = parsed_hostname {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = connector
        .connect(parsed_hostname.unwrap(), socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = stream.unwrap();
    Ok(TokioStream::Tls(Box::new(stream)))
}

#[cfg(all(feature = "smol-rt", feature = "smol-native-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<ClientCert>,
) -> Result<SmolStream> {
    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let socket = stream.unwrap();
    let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));

    let parsed_hostname = ServerName::try_from(host.to_string());

    if let Err(e) = parsed_hostname {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let stream = connector
        .connect(parsed_hostname.unwrap(), socket)
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

#[cfg(all(feature = "smol-rt", feature = "smol-rust-tls"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    _client_cert: &Option<ClientCert>,
) -> Result<SmolStream> {
    use futures_rustls::TlsConnector;
    use rustls::pki_types::ServerName;

    let stream = { TcpStream::connect(format!("{}:{}", host, port)).await };

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tcp {
            host: host.to_string(),
            message: e.to_string(),
        }));
    }

    let socket = stream.unwrap();
    let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS12])
        .expect("Failed to set TLS version")
        .with_root_certificates(root_store)
        .with_no_client_auth();
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
