use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
#[cfg(feature = "http1")]
use hyper::server::conn::http1;
#[cfg(feature = "http2")]
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::server::errors::{EasyHttpMockError, StartError::Tls};
use futures_rustls::TlsAcceptor;
use smol::net::TcpListener;

use crate::rt::task::ServerTask;
use crate::server::tcp::TcpServer;
use crate::server::{Server, ServerConfig};
use crate::utils::generate_port;

pub struct HttpServer {
    port: u16,
    task: Option<ServerTask>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: generate_port(), task: None, config }
    }
}

impl TcpServer for HttpServer {}

impl Server<Incoming, Full<Bytes>> for HttpServer {
    fn port(&self) -> u16 {
        self.port
    }

    async fn start<F, Fut>(&mut self, handler: F) -> Result<(), EasyHttpMockError>
    where
        F: Fn(Request<Incoming>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<Full<Bytes>>, EasyHttpMockError>> + Send + 'static,
    {
        let tls_acceptor = if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err(EasyHttpMockError::Start(Tls(
                    "HttpServer - Server cert and key are required".to_string(),
                )));
            }

            let alpn = if cfg!(feature = "http1") { "http/1.1".into() } else { "h2".into() };

            let tls_config = self.setup_tls(config.clone(), alpn)?;

            Some(TlsAcceptor::from(Arc::new(tls_config)))
        } else {
            None
        };

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| EasyHttpMockError::Bind(e.to_string()))?;

        let handler = Arc::new(service_fn(handler));

        let task = self.handle_connections(listener, tls_acceptor, handler)?;

        self.task = Some(task);

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        if let Some(mut task) = self.task.take() {
            task.cancel().await;
        }
        Ok(())
    }
}
