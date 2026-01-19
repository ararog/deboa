use std::{future::Future, sync::Arc};

use http::{Request, Response};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore,
};

use crate::{
    server::errors::{EasyHttpMockError, StartError::Tls},
    utils::{test_url, CA_CERT},
};

#[cfg(any(feature = "http1", feature = "http2"))]
pub mod tcp;
#[cfg(feature = "http3")]
pub mod udp;

pub mod errors;

#[derive(Clone, Default)]
pub struct ServerConfig {
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    client_auth: Option<bool>,
}

impl ServerConfig {
    pub fn new(cert: Option<Vec<u8>>, key: Option<Vec<u8>>) -> Self {
        Self { cert, key, client_auth: None }
    }

    pub fn cert(&self) -> Option<&Vec<u8>> {
        self.cert.as_ref()
    }

    pub fn key(&self) -> Option<&Vec<u8>> {
        self.key.as_ref()
    }

    pub fn client_auth(&self) -> Option<bool> {
        self.client_auth
    }
}

pub trait Server<RequestBody, ResponseBody> {
    fn port(&self) -> u16;

    fn url(&self, path: &str) -> String {
        format!("{}{}", test_url(Some(self.port())), path)
    }

    fn base_url(&self) -> String {
        test_url(Some(self.port()))
    }

    fn setup_tls(
        &mut self,
        config: ServerConfig,
        alpn_protocols: Vec<u8>,
    ) -> Result<rustls::server::ServerConfig, EasyHttpMockError> {
        let cert = config
            .cert()
            .unwrap()
            .clone();
        let key = config
            .key()
            .unwrap()
            .clone();

        let cert = CertificateDer::from(cert);

        let key = PrivateKeyDer::try_from(key)
            .map_err(|e| EasyHttpMockError::Start(Tls(e.to_string())))?;

        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let builder = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .map_err(|e| EasyHttpMockError::Start(Tls(e.to_string())))?;

        let builder = if config
            .client_auth
            .unwrap_or(false)
        {
            let mut store = RootCertStore::empty();
            let cert = CertificateDer::from(CA_CERT);
            store
                .add(cert)
                .unwrap();

            let client_verifier = WebPkiClientVerifier::builder(Arc::new(store))
                .build()
                .unwrap();
            builder.with_client_cert_verifier(client_verifier)
        } else {
            builder.with_no_client_auth()
        };

        let mut tls_config = builder
            .with_single_cert(vec![cert], key)
            .map_err(|e| EasyHttpMockError::Start(Tls(e.to_string())))?;

        tls_config.max_early_data_size = u32::MAX;
        tls_config.alpn_protocols = vec![alpn_protocols];

        Ok(tls_config)
    }

    fn start<H, Fut>(&mut self, handler: H) -> impl Future<Output = Result<(), EasyHttpMockError>>
    where
        H: Fn(Request<RequestBody>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<ResponseBody>, EasyHttpMockError>> + Send + 'static;

    fn stop(&mut self) -> impl Future<Output = Result<(), EasyHttpMockError>>;
}
