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
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct HttpServer {
    port: u16,
    task: Option<tokio::task::JoinHandle<()>>,
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

        let handle = tokio::spawn(async move {
            loop {
                let (stream, _) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                let io = TokioIo::new(stream);

                tokio::task::spawn(async move {
                    // Handle the connection from the client using HTTP/2 with an executor and pass any
                    // HTTP requests received on that connection to the `hello` function
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
