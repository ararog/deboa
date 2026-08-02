use crate::{
    cert::{Certificate as DeboaCertificate, Identity as DeboaIdentity},
    client::http::conn::stream::create_stream,
};
use compio::net::TcpStream;
use compio_tls::native_tls::TlsConnector;
use cyper_core::HyperStream;
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};
use std::net::IpAddr;

pub(crate) async fn tls_connection(
    ip: IpAddr,
    host: &str,
    port: u16,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<DeboaCertificate>,
    skip_server_verification: bool,
    alpn: &[&str],
) -> Result<HyperStream<TcpStream>> {
    let socket = create_stream(ip, host, port).await?;
    let mut builder = TlsConnector::builder();

    let builder = if skip_server_verification {
        builder
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
    } else {
        &mut builder
    };

    let builder = builder.request_alpns(&alpn);

    let builder = if let Some(ca) = certificate {
        let cert = ca.try_into();
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
        let ident = identity.try_into();
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

    let connector = builder
        .build()
        .map_err(|e| {
            DeboaError::Connection(ConnectionError::Tls {
                host: host.to_owned(),
                message: e.to_string(),
            })
        })?;

    let connector = compio_tls::TlsConnector::from(connector);

    let stream = connector
        .connect(host, socket).await;

    if let Err(e) = stream {
        return Err(DeboaError::Connection(ConnectionError::Tls {
            host: host.to_string(),
            message: format!("Could not connect to server: {}", e),
        }));
    }

    let stream = stream.unwrap();
    Ok(HyperStream::new_tls(stream))
}
