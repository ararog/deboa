use crate::cert::{DeboaCertificate, DeboaIdentity};
use compio::net::TcpStream;
use compio_tls::{
    native_tls::TlsConnector,
    native_tls::{Certificate, Identity},
    TlsStream,
};
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};

#[inline]
pub(crate) fn alpn() -> &'static [&'static str] {
    &[
        #[cfg(feature = "http3")]
        "h3",
        #[cfg(feature = "http2")]
        "h2",
        #[cfg(feature = "http1")]
        "http/1.1",
    ]
}

pub struct TlsConnectionBuilder<'a> {
    tcp_stream: TcpStream,
    host: &'a str,
    identity: Option<&'a DeboaIdentity>,
    certificate: Option<&'a DeboaCertificate>,
    skip_server_verification: bool,
    alpn: &'a [&'a str],
}

impl<'a> TlsConnectionBuilder<'a> {
    pub fn new(tcp_stream: TcpStream, host: &'a str) -> Self {
        Self {
            tcp_stream,
            host,
            identity: None,
            certificate: None,
            skip_server_verification: false,
            alpn: alpn(),
        }
    }

    pub fn identity(mut self, identity: Option<&'a DeboaIdentity>) -> Self {
        self.identity = identity;
        self
    }

    pub fn certificate(mut self, certificate: Option<&'a DeboaCertificate>) -> Self {
        self.certificate = certificate;
        self
    }

    pub fn skip_server_verification(mut self, skip_server_verification: bool) -> Self {
        self.skip_server_verification = skip_server_verification;
        self
    }

    pub fn alpn(mut self, alpn: &'a [&str]) -> Self {
        self.alpn = alpn;
        self
    }

    pub async fn connect(self) -> Result<TlsStream<TcpStream>> {
        let mut builder = TlsConnector::builder();

        let builder = if self.skip_server_verification {
            builder
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
        } else {
            &mut builder
        };

        let builder = builder.request_alpns(self.alpn);

        let builder = if let Some(ca) = self.certificate {
            let cert: Certificate = ca
                .try_into()
                .map_err(|e| {
                    DeboaError::Connection(ConnectionError::Tls {
                        message: format!("Invalid CA certificate: {}", e),
                    })
                })?;
            builder.add_root_certificate(cert)
        } else {
            builder
        };

        let builder = if let Some(identity) = self.identity {
            let ident: Identity = identity
                .try_into()
                .map_err(|e| {
                    DeboaError::Connection(ConnectionError::Tls {
                        message: format!("Invalid client identity: {}", e),
                    })
                })?;
            builder.identity(ident)
        } else {
            builder
        };

        let connector = builder
            .build()
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Tls {
                    message: format!("Could not build TLS connector: {}", e),
                })
            })?;

        let connector = compio_tls::TlsConnector::from(connector);

        let stream = connector
            .connect(self.host, self.tcp_stream)
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Tls {
                    message: format!("Could not connect to server: {}", e),
                })
            })?;

        Ok(stream)
    }
}
