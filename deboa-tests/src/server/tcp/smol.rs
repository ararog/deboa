use std::net::SocketAddr;
use std::sync::Arc;

use bytes::Bytes;
use futures_rustls::TlsAcceptor;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body;
#[cfg(feature = "http1")]
use hyper::{server::conn::http1, service::service_fn};
#[cfg(feature = "http2")]
use hyper::{server::conn::http2, service::service_fn};
use rustls::pki_types::pem::PemObject;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use smol::net::TcpListener;
use smol_hyper::rt::FuturesIo;

use crate::server::ServerConfig;
use crate::utils::{generate_port, test_url};

pub struct HttpServer {
    port: u16,
    task: Option<smol::Task<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: generate_port(), task: None, config }
    }
}

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", test_url(Some(self.port)), path)
    }

    pub fn base_url(&self) -> String {
        test_url(Some(self.port))
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self
            .config
            .is_none()
        {
            return Err("HttpServer - Server config is required".into());
        }

        let tls_acceptor = if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("HttpServer - Server cert and key are required".into());
            }

            let cert = config
                .cert()
                .unwrap()
                .clone();
            let key = config
                .key()
                .unwrap()
                .clone();

            let cert = CertificateDer::from_pem_slice(&cert);
            if let Err(e) = cert {
                eprintln!("HttpServer - Error loading cert: {}", e);
                return Err(e.into());
            }

            let cert = cert.unwrap();

            let key = PrivateKeyDer::from_pem_slice(&key);
            if let Err(e) = key {
                eprintln!("HttpServer - Error loading private key: {}", e);
                return Err(e.into());
            }

            let key = key.unwrap();

            let provider = rustls::crypto::aws_lc_rs::default_provider();
            let mut tls_config = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .expect("HttpServer - Failed to set TLS version")
                .with_no_client_auth()
                .with_single_cert(vec![cert], key)?;

            tls_config.max_early_data_size = u32::MAX;
            if cfg!(feature = "http1") {
                tls_config.alpn_protocols = vec![b"http/1.1".to_vec()];
            } else if cfg!(feature = "http2") {
                tls_config.alpn_protocols = vec![b"h2".to_vec()];
            }

            Some(TlsAcceptor::from(Arc::new(tls_config)))
        } else {
            None
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let listener = TcpListener::bind(addr).await?;

        let handle = smol::spawn(async move {
            loop {
                let result = listener
                    .accept()
                    .await;

                let stream = match result {
                    Ok((stream, _)) => stream,
                    Err(err) => {
                        eprintln!("HttpServer - Error accepting connection: {}", err);
                        return;
                    }
                };

                if let Some(acceptor) = &tls_acceptor {
                    let tls_stream = acceptor
                        .accept(stream)
                        .await
                        .expect("HttpServer - Failed to accept TLS connection");
                    let io = FuturesIo::new(tls_stream);
                    smol::spawn(async move {
                        #[cfg(feature = "http1")]
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(SmolExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                    })
                    .detach();
                } else {
                    let io = FuturesIo::new(stream);
                    smol::spawn(async move {
                        #[cfg(feature = "http1")]
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(SmolExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                    })
                    .detach();
                };
            }
        });

        self.task = Some(handle);

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(task) = self.task.take() {
            task.cancel().await;
        }
    }
}

use hyper::rt::Executor;
use std::future::Future;

#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct SmolExecutor {}

impl<Fut> Executor<Fut> for SmolExecutor
where
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        smol::spawn(fut).detach();
    }
}

impl SmolExecutor {
    pub fn new() -> Self {
        Self {}
    }
}
