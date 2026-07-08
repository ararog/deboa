use std::sync::Arc;

use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ClientConfig;

use crate::cert::Certificate;
use crate::{
    cert::Identity as DeboaIdentity,
    errors::{ConnectionError, DeboaError},
    Result,
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

pub(crate) fn default_provider() -> Arc<rustls::crypto::CryptoProvider> {
    #[cfg(feature = "__rustls_aws_lc_rs")]
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    #[cfg(feature = "__rustls_ring")]
    let provider = rustls::crypto::ring::default_provider();
    #[cfg(feature = "__rustls_rustcrypto")]
    let provider = rustls_rustcrypto::provider();
    Arc::new(provider)
}

pub fn setup_rust_tls(
    host: &str,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<Certificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<ClientConfig> {
    let provider = default_provider();

    if skip_server_verification {
        use crate::client::conn::rustls::verify::SkipServerVerification;
        let config = rustls::ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("Failed to set TLS version")
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();
        return Ok(config);
    }

    #[cfg(feature = "__webpki_rustls_verifier")]
    let config = {
        let config = rustls::ClientConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("Failed to set TLS version");

        let mut root_store =
            rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
        if let Some(ca) = certificate {
            let cert = ca.try_into();
            if let Err(e) = cert {
                return Err(DeboaError::Connection(ConnectionError::Tls {
                    host: host.to_string(),
                    message: format!("Invalid CA certificate: {}", e),
                }));
            }

            let result = root_store.add(cert.unwrap());
            if let Err(e) = result {
                return Err(DeboaError::Connection(ConnectionError::Tls {
                    host: host.to_string(),
                    message: format!("Could not add CA certificate to the store: {}", e),
                }));
            }

            config.with_root_certificates(root_store)
        } else {
            config.with_root_certificates(root_store)
        }
    };

    #[cfg(feature = "__platform_rustls_verifier")]
    let config = {
        use rustls_platform_verifier::Verifier;
        let verifier = Verifier::new(provider).expect("Failed to create platform verifier");
        rustls::ClientConfig::builder_with_provider(default_provider())
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("Failed to set TLS version")
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(verifier))
    };

    let mut config = if let Some(id) = identity {
        let pair: std::result::Result<
            (CertificateDer<'static>, PrivateKeyDer<'static>),
            std::io::Error,
        > = id.try_into();
        if let Err(e) = pair {
            return Err(DeboaError::Connection(ConnectionError::Tls {
                host: host.to_string(),
                message: format!("Invalid client identity: {}", e),
            }));
        }

        let pair = pair.unwrap();

        config
            .with_client_auth_cert(vec![pair.0], pair.1)
            .expect("Failed to set client identity")
    } else {
        config.with_no_client_auth()
    };

    config.enable_early_data = true;

    config.alpn_protocols = alpn;

    Ok(config)
}

pub(crate) mod verify {
    use std::sync::Arc;

    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};

    #[derive(Debug)]
    pub(crate) struct SkipServerVerification(Arc<rustls::crypto::CryptoProvider>);

    impl SkipServerVerification {
        pub(crate) fn new() -> Arc<Self> {
            let provider = super::default_provider();
            Arc::new(Self(provider))
        }
    }

    impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp: &[u8],
            _now: UnixTime,
        ) -> std::result::Result<rustls::client::danger::ServerCertVerified, rustls::Error>
        {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &rustls::DigitallySignedStruct,
        ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
        {
            rustls::crypto::verify_tls12_signature(
                message,
                cert,
                dss,
                &self
                    .0
                    .signature_verification_algorithms,
            )
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &rustls::DigitallySignedStruct,
        ) -> std::result::Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error>
        {
            rustls::crypto::verify_tls13_signature(
                message,
                cert,
                dss,
                &self
                    .0
                    .signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            self.0
                .signature_verification_algorithms
                .supported_schemes()
        }
    }
}
