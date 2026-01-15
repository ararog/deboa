use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
#[cfg(feature = "http2")]
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;

use crate::server::tcp::TcpServer;
use crate::server::{BoxedError, Server, ServerConfig};
use crate::utils::generate_port;

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

impl TcpServer for HttpServer {}

impl Server<Incoming, Full<Bytes>> for HttpServer {
    type RequestFunction = fn(Request<Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>;

    fn port(&self) -> u16 {
        self.port
    }

    async fn start(&mut self, handler: Self::RequestFunction) -> Result<(), BoxedError> {
        let tls_acceptor = if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err("HttpServer - Server cert and key are required".into());
            }

            let alpn = if cfg!(feature = "http1") { "http/1.1".into() } else { "h2".into() };

            let tls_config = self.setup_tls(config.clone(), alpn)?;

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
                    .expect("HttpServer - Failed to accept connection");

                if let Some(acceptor) = &tls_acceptor {
                    let tls_stream = acceptor
                        .accept(stream)
                        .await
                        .expect("HttpServer - Failed to accept TLS connection");

                    let io = TokioIo::new(tls_stream);
                    tokio::task::spawn(async move {
                        #[cfg(feature = "http1")]
                        if let Err(err) = http1::Builder::new()
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(TokioExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
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
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                        #[cfg(feature = "http2")]
                        if let Err(err) = http2::Builder::new(TokioExecutor::new())
                            .serve_connection(io, service_fn(|req| async move { handler(req) }))
                            .await
                        {
                            eprintln!("HttpServer - Error serving connection: {}", err);
                        }
                    });
                }
            }
        });

        self.task = Some(handle);

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(task) = &self.task.take() {
            task.abort();
        }
        Ok(())
    }
}
