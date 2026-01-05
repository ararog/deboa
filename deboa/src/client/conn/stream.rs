#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3-tokio"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
))]
use std::{fs::File, io::BufReader, sync::Arc};

#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
use rustls::ClientConfig;

#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3-tokio"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
))]
use crate::{
    cert::Identity as DeboaIdentity,
    errors::{ConnectionError, DeboaError},
    Result,
};

#[cfg(all(
    any(feature = "http1", feature = "http2", feature = "http3-tokio"),
    any(feature = "tokio-rust-tls", feature = "smol-rust-tls")
))]
pub fn setup_rust_tls(
    host: &str,
    client_cert: &Option<DeboaIdentity>,
    skip_server_verification: bool,
) -> Result<ClientConfig> {
    let root_store = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
    let provider = rustls::crypto::aws_lc_rs::default_provider();
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

#[cfg(all(
    any(
        all(feature = "tokio-rt", feature = "tokio-rust-tls"),
        all(feature = "smol-rt", feature = "smol-rust-tls")
    ),
    any(feature = "http1", feature = "http2", feature = "http3-tokio")
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
