use std::net::SocketAddr;

use bytes::Bytes;
use http::{Request, Response};
use http_body_util::Full;
use hyper::body;
#[cfg(feature = "http1")]
use hyper::{server::conn::http1, service::service_fn};
#[cfg(feature = "http2")]
use hyper::{server::conn::http2, service::service_fn};
use smol::net::TcpListener;
use smol_hyper::rt::FuturesIo;

pub struct HttpServer {
    port: u16,
    task: Option<smol::Task<()>>,
    config: Option<ServerConfig>,
}

impl HttpServer {
    pub fn new(config: Option<ServerConfig>) -> Self {
        Self { port: rand::random_range(20000..65535), task: None, config }
    }
}

impl HttpServer {
    pub fn url(&self, path: &str) -> String {
        format!("http://localhost:{}{}", self.port, path)
    }

    pub async fn start(
        &mut self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                        eprintln!("Error accepting connection: {}", err);
                        return;
                    }
                };

                let io = FuturesIo::new(stream);

                smol::spawn(async move {
                    #[cfg(feature = "http1")]
                    if let Err(err) = http1::Builder::new()
                        .serve_connection(io, service_fn(|req| async move { handler(req) }))
                        .await
                    {
                        eprintln!("Error serving connection: {}", err);
                    }
                    #[cfg(feature = "http2")]
                    if let Err(err) = http2::Builder::new(SmolExecutor::new())
                        .serve_connection(io, service_fn(|req| async move { handler(req) }))
                        .await
                    {
                        eprintln!("Error serving connection: {}", err);
                    }
                })
                .detach();
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
