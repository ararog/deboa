use std::sync::Arc;

use crate::rt::stream::TokioStream;
use tokio::net::TcpStream;
use trust_dns_resolver::error::ResolveErrorKind;

use crate::client::conn::rustls::setup_rust_tls;
use rustls::pki_types::ServerName;
use tokio_rustls::TlsConnector;

use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};

use crate::{cert::Certificate as DeboaCertificate, cert::Identity as DeboaIdentity};

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

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

pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<TokioStream> {
    let stream = create_stream(host, port).await?;
    Ok(TokioStream::Plain(stream))
}

pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<TokioStream> {
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
    Ok(TokioStream::Tls(Box::new(stream)))
}
