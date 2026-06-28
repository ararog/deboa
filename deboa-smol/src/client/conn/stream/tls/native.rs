use crate::{
    cert::{Certificate as DeboaCertificate, Identity as DeboaIdentity},
    rt::{plain::create_stream, stream::SmolStream},
};
use async_native_tls::{Certificate, Identity, TlsConnector};
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};

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
