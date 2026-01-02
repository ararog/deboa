#[cfg(all(
    any(
        all(feature = "tokio-rt", feature = "tokio-rust-tls"),
        all(feature = "smol-rt", feature = "smol-rust-tls")
    ),
    any(feature = "http1", feature = "http2")
))]
use std::{fs::File, io::BufReader, sync::Arc};

#[cfg(feature = "smol-rt")]
use crate::rt::smol::stream::SmolStream;
#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use crate::rt::tokio::stream::TokioStream;
use rustls::ClientConfig;
#[cfg(feature = "smol-rt")]
use smol::net::TcpStream;
#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
use tokio::net::TcpStream;

#[cfg(feature = "smol-native-tls")]
use async_native_tls::{Certificate, Identity, TlsConnector};
#[cfg(feature = "tokio-native-tls")]
use tokio_native_tls::native_tls::{Certificate, Identity, TlsConnector};

#[cfg(feature = "smol-rust-tls")]
use futures_rustls::TlsConnector;
#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
#[cfg(all(feature = "tokio-rust-tls", any(feature = "http1", feature = "http2")))]
use tokio_rustls::TlsConnector;

use crate::{
    cert::Identity,
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

#[cfg(all(any(feature = "http1", feature = "http2"), feature = "tokio-rust-tls"))]
fn setup_rust_tls(host: &str, client_cert: &Option<Identity>) -> Result<ClientConfig> {
    let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS12])
        .expect("Failed to set TLS version");

    let config = if let Some(client_cert) = client_cert {
        let config = if let Some(ca) = client_cert.ca() {
            let file = File::open(ca);
            if file.is_err() {
                return Err(DeboaError::Connection(ConnectionError::Tls {
                    host: host.to_string(),
                    message: format!("Failed to open CA file: {}", ca),
                }));
            }
            let mut ca_file = BufReader::new(file.unwrap());
            let mut root_store = rustls::RootCertStore::empty();
            let certs = root_store.add_parsable_certificates(
                rustls_pemfile::certs(&mut ca_file).filter_map(|c| c.ok()),
            );
            if certs.0 == 0 {
                return Err(DeboaError::Connection(ConnectionError::Tls {
                    host: host.to_string(),
                    message: format!("No valid certificates found in CA file: {}", ca),
                }));
            }
            config.with_root_certificates(root_store)
        } else {
            config.with_root_certificates(root_store)
        };

        let file = File::open(client_cert.cert());
        if file.is_err() {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: format!("Failed to open client certificate file: {}", client_cert.cert()),
            }));
        }

        let mut cert_file = BufReader::new(file.unwrap());
        let cert_chain: Vec<CertificateDer<'static>> = rustls_pemfile::certs(&mut cert_file)
            .filter_map(|c| c.ok())
            .collect();

        let file = File::open(client_cert.cert());
        if file.is_err() {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: format!("Failed to open client certificate file: {}", client_cert.cert()),
            }));
        }

        let mut key_file = BufReader::new(file.unwrap());
        let pkcs8_bytes = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .filter_map(|k| k.ok())
            .next()
            .expect("Could not find PKCS#8 private key");
        let client_key = PrivateKeyDer::Pkcs8(pkcs8_bytes);
        config
            .with_client_auth_cert(cert_chain, client_key)
            .unwrap()
    } else {
        config
            .with_root_certificates(root_store)
            .with_no_client_auth()
    };

    Ok(config)
}

#[cfg(all(feature = "tokio-rt", any(feature = "http1", feature = "http2")))]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<TokioStream> {
    let stream = create_stream(host, port).await?;
    Ok(TokioStream::Plain(stream))
}

#[cfg(all(feature = "smol-rt", any(feature = "http1", feature = "http2")))]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<SmolStream> {
    let stream = create_stream(host, port).await?;
    Ok(SmolStream::Plain(stream))
}

#[cfg(all(
    feature = "tokio-rt",
    any(feature = "http1", feature = "http2"),
    feature = "tokio-native-tls"
))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<ClientCert>,
) -> Result<TokioStream> {
    let socket = create_stream(host, port).await?;
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

#[cfg(all(
    feature = "tokio-rt",
    any(feature = "http1", feature = "http2"),
    feature = "tokio-rust-tls"
))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<Identity>,
) -> Result<TokioStream> {
    let socket = create_stream(host, port).await?;
    let config = setup_rust_tls(host, client_cert)?;
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
    Ok(TokioStream::Tls(Box::new(stream)))
}

#[cfg(all(
    feature = "smol-rt",
    any(feature = "http1", feature = "http2"),
    feature = "smol-native-tls"
))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<Identity>,
) -> Result<SmolStream> {
    let socket = create_stream(host, port).await?;
    let connector = if let Some(client_cert) = client_cert {
        let builder = TlsConnector::new();
        if let Some(ca) = client_cert.ca() {
            let pem = std::fs::read(ca);
            if let Err(e) = pem {
                return Err(DeboaError::ClientCert { message: e.to_string() });
            }
            let cert = Certificate::from_pem(&pem.unwrap());
            builder.add_root_certificate(cert.unwrap());
        }

        let file = std::fs::read(client_cert.cert());
        if let Err(e) = file {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }
        let identity = Identity::from_pkcs12(&file.unwrap(), client_cert.pw());
        if let Err(e) = identity {
            return Err(DeboaError::ClientCert { message: e.to_string() });
        }

        builder.identity(identity.unwrap())
    } else {
        TlsConnector::new()
    };

    let stream = connector
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

#[cfg(all(
    feature = "smol-rt",
    any(feature = "http1", feature = "http2"),
    feature = "smol-rust-tls"
))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    client_cert: &Option<Identity>,
) -> Result<SmolStream> {
    let socket = create_stream(host, port).await?;
    let config = setup_rust_tls(host, client_cert)?;
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
