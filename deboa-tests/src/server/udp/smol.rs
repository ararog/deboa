use std::future::Future;
use std::{net::SocketAddr, sync::Arc};

use bytes::Bytes;
use h3::server::RequestStream;
use h3_quinn::quinn::{self, crypto::rustls::QuicServerConfig};
use http::{Request, Response};
use http_body_util::Full;
use hyper::service::service_fn;

use crate::rt::task::ServerTask;
use crate::server::errors::{EasyHttpMockError, StartError::Tls};
use crate::server::udp::UdpServer;
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

impl UdpServer for HttpServer {}

impl Server<Full<Bytes>, Full<Bytes>> for HttpServer {
    fn port(&self) -> u16 {
        self.port
    }

    async fn start<H, Fut>(&mut self, handler: H) -> Result<(), EasyHttpMockError>
    where
        H: Fn(Request<Full<Bytes>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response<Full<Bytes>>, EasyHttpMockError>> + Send + 'static,
    {
        if let Some(config) = &self.config {
            if config
                .cert
                .is_none()
                || config.key.is_none()
            {
                return Err(EasyHttpMockError::Start(Tls(
                    "HttpServer - Server cert and key are required".to_string(),
                )));
            }

            let tls_config = self.setup_tls(config.clone(), b"h3".to_vec())?;

            let quic_config = QuicServerConfig::try_from(tls_config)
                .map_err(|e| EasyHttpMockError::Start(Tls(e.to_string())))?;

            let server_config = quinn::ServerConfig::with_crypto(Arc::new(quic_config));
            let endpoint = quinn::Endpoint::server(
                server_config,
                SocketAddr::from(([127, 0, 0, 1], self.port)),
            )
            .map_err(|e| EasyHttpMockError::Bind(e.to_string()))?;

            let handler = Arc::new(service_fn(handler));

            let server_task = self.handle_connections(endpoint, handler)?;

            self.task = Some(server_task);
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<(), EasyHttpMockError> {
        if let Some(mut task) = self.task.take() {
            task.cancel().await;
        }
        Ok(())
    }
}
