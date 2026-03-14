#[cfg(any(feature = "http1", feature = "http2"))]
use crate::rt::tokio::stream::TokioStream;
#[cfg(any(feature = "http1", feature = "http2"))]
use tokio::net::TcpStream;
#[cfg(any(feature = "http1", feature = "http2"))]
use trust_dns_resolver::error::ResolveErrorKind;

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
use async_native_tls::{Certificate, Identity, TlsConnector};

#[cfg(any(feature = "http1", feature = "http2"))]
use crate::{
    cert::Certificate as DeboaCertificate,
    cert::Identity as DeboaIdentity,
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(any(feature = "http1", feature = "http2"))]
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

#[cfg(any(feature = "http1", feature = "http2"))]
async fn create_stream(host: &str, port: u16) -> Result<TcpStream> {
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
                return Err(DeboaError::Connection(ConnectionError::Tcp {
                    host: host.to_string(),
                    message: format!("Could not resolve host: {}", query_name),
                }));
            }
            _ => {
                return Err(DeboaError::Connection(ConnectionError::Tcp {
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

    let tcp_stream = TcpStream::connect((addr, port)).await;
    let tcp_stream = match tcp_stream {
        Ok(tcp_stream) => tcp_stream,
        Err(e) => {
            return Err(DeboaError::Connection(ConnectionError::Tcp {
                host: host.to_string(),
                message: format!("Could not connect to server: {}", e),
            }));
        }
    };

    Ok(tcp_stream)
}

#[cfg(any(feature = "http1", feature = "http2"))]
pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<TokioStream> {
    let stream = create_stream(host, port).await?;
    Ok(TokioStream::Plain(stream))
}

#[cfg(any(feature = "http1", feature = "http2"))]
pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: &[&str],
) -> Result<TokioStream> {
    let socket = create_stream(host, port).await?;
    let builder = TlsConnector::new();

    let builder = if skip_server_verification {
        builder
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
    } else {
        builder
    };

    let builder = builder.request_alpns(alpn);

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
    Ok(TokioStream::Tls(stream))
}
