use crate::rt::stream::CompioStream;
use compio::net::TcpStream;
use std::sync::Arc;

use crate::client::conn::rustls::setup_rust_tls;
use compio_tls::TlsConnector;
use rustls::pki_types::ServerName;
use trust_dns_resolver::error::ResolveErrorKind;

use crate::{
    cert::{Certificate as DeboaCertificate, Identity as DeboaIdentity},
    errors::{ConnectionError, DeboaError},
    Result,
};

use async_std_resolver::{
    config::{ResolverConfig, ResolverOpts},
    resolver,
};

async fn create_stream(host: &str, port: u16) -> Result<TcpStream> {
    let resolver = resolver(ResolverConfig::default(), ResolverOpts::default()).await;

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

pub(crate) async fn plain_connection(host: &str, port: u16) -> Result<CompioStream> {
    let stream = create_stream(host, port).await?;
    Ok(CompioStream::Plain(stream))
}

pub(crate) async fn tls_connection(
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<CompioStream> {
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
    let hostname = hostname.unwrap();

    let stream = connector
        .connect(&hostname.to_str(), socket)
        .await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: format!("Could not connect to server: {}", e),
        }));
    }

    let stream = stream.unwrap();
    Ok(CompioStream::Tls(stream))
}
