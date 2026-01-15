use std::{future::Future, sync::Arc};

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body::Incoming;
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    RootCertStore,
};

use crate::utils::{test_url, CA_CERT};

#[cfg(any(feature = "http1", feature = "http2"))]
pub mod tcp;
#[cfg(feature = "http3")]
pub mod udp;

pub type Http1RequestHandler =
    fn(Request<Incoming>) -> std::result::Result<Response<Full<Bytes>>, hyper::Error>;
pub type Http2RequestHandler =
    fn(Request<Incoming>) -> std::result::Result<Response<Full<Bytes>>, hyper::Error>;
pub type Http3RequestHandler =
    fn(Request<Full<Bytes>>) -> std::result::Result<Response<Full<Bytes>>, hyper::Error>;

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

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub trait Server<RequestBody, ResponseBody> {
    type RequestFunction: Fn(Request<RequestBody>) -> Result<Response<ResponseBody>, hyper::Error>;

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
    ) -> Result<rustls::server::ServerConfig, BoxedError> {
        let cert = config
            .cert()
            .unwrap()
            .clone();
        let key = config
            .key()
            .unwrap()
            .clone();

        let cert = CertificateDer::from(cert);

        let key = PrivateKeyDer::try_from(key);
        if let Err(e) = key {
            eprintln!("HttpServer - Error loading private key: {}", e);
            return Err(e.into());
        }

        let key = key.unwrap();

        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let builder = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("HttpServer - Failed to set TLS version");

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

        let mut tls_config = builder.with_single_cert(vec![cert], key)?;

        tls_config.max_early_data_size = u32::MAX;
        tls_config.alpn_protocols = vec![alpn_protocols];

        Ok(tls_config)
    }

    fn start(
        &mut self,
        handler: Self::RequestFunction,
    ) -> impl Future<Output = Result<(), BoxedError>>;

    fn stop(&mut self) -> impl Future<Output = Result<(), BoxedError>>;
}
