use http_body_util::Full;
use hyper::body::{self, Bytes};
#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
#[cfg(feature = "http2")]
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::server::ServerConfig;
use crate::utils::{generate_port, TEST_HOST};

pub struct HttpServer {
    port: u16,
    task: Option<tokio::task::JoinHandle<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: generate_port(), task: None, config }
    }
}

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("{}:{}{}", TEST_HOST, self.port, path)
    }

    pub fn base_url(&self) -> String {
        format!("{}:{}", TEST_HOST, self.port)
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self
            .config
            .is_none()
        {
            return Err("Server config is required".into());
        }

        let tls_acceptor = if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("Server cert and key are required".into());
            }

            let cert = CertificateDer::from(std::fs::read(
                config
                    .cert
                    .as_ref()
                    .unwrap(),
            )?);
            let key = PrivateKeyDer::try_from(std::fs::read(
                config
                    .key
                    .as_ref()
                    .unwrap(),
            )?)?;

            let provider = rustls::crypto::aws_lc_rs::default_provider();
            let mut tls_config = rustls::ServerConfig::builder_with_provider(Arc::new(provider))
                .with_protocol_versions(&[&rustls::version::TLS13])
                .expect("Failed to set TLS version")
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

        let handle = tokio::spawn(async move {
            loop {
                let (stream, _) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                if let Some(acceptor) = &tls_acceptor {
                    let tls_stream = acceptor
                        .accept(stream)
                        .await
                        .expect("Failed to accept TLS connection");
                    let io = TokioIo::new(tls_stream);
                    tokio::task::spawn(async move {
                        #[cfg(feature = "http1")]
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(TokioExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("Error serving connection: {}", err);
                        }
                    });
                } else {
                    let io = TokioIo::new(stream);
                    tokio::task::spawn(async move {
                        #[cfg(feature = "http1")]
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(TokioExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("Error serving connection: {}", err);
                        }
                    });
                }
            }
        });

        self.task = Some(handle);

        Ok(())
    }

    pub async fn stop(&self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}
