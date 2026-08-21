//! TLS implementation using rustls

use crate::cert::{DeboaCertificate, DeboaIdentity};
use deboa::{
    errors::{ConnectionError, DeboaError},
    Result,
};
use rustls::{
    crypto::CryptoProvider,
    pki_types::{CertificateDer, PrivateKeyDer},
    ClientConfig,
};

pub(crate) fn default_provider() -> CryptoProvider {
    #[cfg(feature = "__rustls_aws_lc_rs")]
    return rustls::crypto::aws_lc_rs::default_provider();
    #[cfg(feature = "__rustls_ring")]
    return rustls::crypto::ring::default_provider();
}

#[inline]
pub(crate) fn alpn() -> Vec<Vec<u8>> {
    vec![
        #[cfg(feature = "http3")]
        b"h3".to_vec(),
        #[cfg(feature = "http2")]
        b"h2".to_vec(),
        #[cfg(feature = "http1")]
        b"http/1.1".to_vec(),
    ]
}

/// Builder for TLS connections using rustls
pub struct TlsConnectionBuilder<'a> {
    identity: Option<&'a DeboaIdentity>,
    certificate: Option<&'a DeboaCertificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
    provider: CryptoProvider,
}

impl Default for TlsConnectionBuilder<'_> {
    fn default() -> Self {
        Self {
            identity: None,
            certificate: None,
            skip_server_verification: false,
            alpn: alpn(),
            provider: default_provider(),
        }
    }
}

impl<'a> TlsConnectionBuilder<'a> {
    /// Set the identity to use for the connection
    pub fn identity(mut self, identity: Option<&'a DeboaIdentity>) -> Self {
        self.identity = identity;
        self
    }

    /// Set the certificate to use for the connection
    pub fn certificate(mut self, certificate: Option<&'a DeboaCertificate>) -> Self {
        self.certificate = certificate;
        self
    }

    /// Skip server verification
    pub fn skip_server_verification(mut self, skip_server_verification: bool) -> Self {
        self.skip_server_verification = skip_server_verification;
        self
    }

    /// Set the ALPN protocols to use for the connection
    pub fn alpn(mut self, alpn: Vec<Vec<u8>>) -> Self {
        self.alpn = alpn;
        self
    }

    /// Build the TLS client configuration
    pub fn build_config(self) -> Result<ClientConfig> {
        let client_config = {
            if self.skip_server_verification {
                ClientConfig::builder()
                    .dangerous()
                    .with_custom_certificate_verifier(
                        deboa_tls::rust::verify::SkipServerVerification::new(self.provider),
                    )
                    .with_no_client_auth()
            } else {
                #[cfg(feature = "__webpki_rustls_verifier")]
                let config = {
                    let config = ClientConfig::builder_with_provider(self.provider.into())
                        .with_protocol_versions(rustls::ALL_VERSIONS)
                        .map_err(|e| {
                            DeboaError::Connection(ConnectionError::Tls {
                                message: format!("Failed to set TLS version: {}", e),
                            })
                        })?;

                    let mut root_store =
                        rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
                    let config = if let Some(ca) = self.certificate {
                        let cert = ca
                            .try_into()
                            .map_err(|e| {
                                DeboaError::Connection(ConnectionError::Tls {
                                    message: format!("Invalid CA certificate: {}", e),
                                })
                            })?;

                        root_store
                            .add(cert)
                            .map_err(|e| {
                                DeboaError::Connection(ConnectionError::Tls {
                                    message: format!(
                                        "Could not add CA certificate to the store: {}",
                                        e
                                    ),
                                })
                            })?;

                        config.with_root_certificates(root_store)
                    } else {
                        config.with_root_certificates(root_store)
                    };

                    config
                };

                #[cfg(feature = "__platform_rustls_verifier")]
                let config = {
                    use rustls_platform_verifier::BuilderVerifierExt;
                    rustls::ClientConfig::builder_with_provider(default_provider())
                        .with_protocol_versions(rustls::ALL_VERSIONS)
                        .map_err(|e| {
                            DeboaError::Connection(ConnectionError::Tls {
                                message: format!("Failed to set TLS version: {}", e),
                            })
                        })?
                        .with_platform_verifier()
                };

                let mut config = if let Some(id) = self.identity {
                    let pair: (CertificateDer<'_>, PrivateKeyDer<'_>) = id
                        .try_into()
                        .map_err(|e| {
                            DeboaError::Connection(ConnectionError::Tls {
                                message: format!("Invalid client identity: {}", e),
                            })
                        })?;

                    config
                        .with_client_auth_cert(vec![pair.0], pair.1)
                        .map_err(|e| {
                            DeboaError::Connection(ConnectionError::Tls {
                                message: format!("Failed to set client identity: {}", e),
                            })
                        })?
                } else {
                    config.with_no_client_auth()
                };

                config.enable_early_data = true;

                config.alpn_protocols = self.alpn;

                config
            }
        };

        Ok(client_config)
    }
}

#[cfg(any(feature = "http1", feature = "http2"))]
/// TCP connection module for TLS
pub mod tcp {
    use deboa::{
        errors::{ConnectionError, DeboaError},
        Result,
    };
    use rustls::ClientConfig;
    use rustls_pki_types::ServerName;
    use std::sync::Arc;
    use tokio::net::TcpStream;
    use tokio_rustls::{client::TlsStream, TlsConnector};

    /// Establish a TLS connection over TCP
    pub async fn connect(
        config: ClientConfig,
        inner_stream: TcpStream,
        host: &str,
    ) -> Result<TlsStream<TcpStream>> {
        let connector = TlsConnector::from(Arc::new(config));

        let hostname = ServerName::try_from(host.to_string())
            .map_err(|e| DeboaError::Connection(ConnectionError::Tls { message: e.to_string() }))?;

        connector
            .connect(hostname, inner_stream)
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Tls {
                    message: format!("Could not connect to server: {}", e),
                })
            })
    }
}

#[cfg(feature = "http3")]
/// UDP connection module for TLS
pub mod udp {
    use deboa::{
        errors::{http::ConnectionError, DeboaError},
        Result,
    };
    use h3_quinn::Connection;
    use quinn::{crypto::rustls::QuicClientConfig, Endpoint};
    use rustls::ClientConfig;
    use std::{net::SocketAddr, sync::Arc};

    /// Establish a TLS connection over UDP
    pub async fn connect(
        config: ClientConfig,
        endpoint: &mut Endpoint,
        socket_addr: SocketAddr,
        host: &str,
    ) -> Result<Connection> {
        let quic_config = QuicClientConfig::try_from(config).map_err(|e| {
            DeboaError::Connection(ConnectionError::Tls {
                message: format!("Could not create QUIC client config: {}", e),
            })
        })?;

        let client_config = quinn::ClientConfig::new(Arc::new(quic_config));
        endpoint.set_default_client_config(client_config);

        let conn = endpoint
            .connect(socket_addr, host)
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Udp {
                    message: format!("Could not connect to server: {}", e),
                })
            })?;

        let conn = conn
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Udp {
                    message: format!("Could not connect to server: {}", e),
                })
            })?;

        let quinn_conn = h3_quinn::Connection::new(conn);

        Ok(quinn_conn)
    }
}
