#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls", feature = "compio-rust-tls")
))]
use std::sync::Arc;

#[cfg(any(
    feature = "tokio-rust-tls",
    feature = "smol-rust-tls",
    feature = "compio-rust-tls"
))]
use rustls::ClientConfig;

#[cfg(any(
    feature = "tokio-rust-tls",
    feature = "smol-rust-tls",
    feature = "compio-rust-tls"
))]
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls", feature = "compio-rust-tls")
))]
use crate::cert::Certificate;
#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls", feature = "compio-rust-tls")
))]
use crate::{
    cert::Identity as DeboaIdentity,
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls", feature = "compio-rust-tls")
))]
pub fn setup_rust_tls(
    host: &str,
    identity: &Option<DeboaIdentity>,
    certificate: &Option<Certificate>,
    skip_server_verification: bool,
    alpn: Vec<Vec<u8>>,
) -> Result<ClientConfig> {
    let mut root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    #[cfg(feature = "__rustls_aws_lc_rs")]
    let provider = rustls::crypto::aws_lc_rs::default_provider();
    #[cfg(feature = "__rustls_ring")]
    let provider = rustls::crypto::ring::default_provider();
    #[cfg(feature = "__rustls_rustcrypto")]
    let provider = rustls_rustcrypto::provider();

    let config = rustls::ClientConfig::builder_with_provider(Arc::new(provider))
        .with_protocol_versions(&[&rustls::version::TLS13])
        .expect("Failed to set TLS version");

    if skip_server_verification {
        use crate::client::conn::stream::verify::SkipServerVerification;

        let config = config
            .dangerous()
            .with_custom_certificate_verifier(SkipServerVerification::new())
            .with_no_client_auth();
        return Ok(config);
    }

    let config = if let Some(ca) = certificate {
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

#[cfg(all(
    any(
        all(feature = "tokio-rt", feature = "tokio-rust-tls"),
        all(feature = "smol-rt", feature = "smol-rust-tls"),
        all(feature = "compio-rt", feature = "compio-rust-tls"),
    ),
    any(feature = "http1", feature = "http2", feature = "http3")
))]
pub(crate) mod verify {
    use std::sync::Arc;

    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};

    #[derive(Debug)]
    pub(crate) struct SkipServerVerification(Arc<rustls::crypto::CryptoProvider>);

    impl SkipServerVerification {
        pub(crate) fn new() -> Arc<Self> {
            Arc::new(Self(Arc::new(rustls::crypto::aws_lc_rs::default_provider())))
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
