use http_body_util::Full;
use hyper::body::{self, Bytes};
use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub struct HttpServer {
    port: u16,
}

impl HttpServer {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

impl HttpServer {
    pub async fn start(
        &self,
        handler: fn(Request<body::Incoming>) -> Result<Response<Full<Bytes>>, hyper::Error>,
    ) -> Result<Infallible, Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        // Bind to the port and listen for incoming TCP connections
        let listener = TcpListener::bind(addr).await?;

        loop {
            // When an incoming TCP connection is received grab a TCP stream for
            // client-server communication.
            //
            // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
            // .await point allows the Tokio runtime to pull the task off of the thread until the task
            // has work to do. In this case, a connection arrives on the port we are listening on and
            // the task is woken up, at which point the task is then put back on a thread, and is
            // driven forward by the runtime, eventually yielding a TCP stream.
            let (stream, _) = listener
                .accept()
                .await
                .expect("Failed to accept connection");
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
            // current task without waiting for the processing of the HTTP/2 connection we just received
            // to finish
            tokio::task::spawn(async move {
                // Handle the connection from the client using HTTP/2 with an executor and pass any
                // HTTP requests received on that connection to the `hello` function
                if let Err(err) = http2::Builder::new(TokioExecutor::new())
                    .serve_connection(io, service_fn(|req| async move { handler(req) }))
                    .await
                {
                    eprintln!("Error serving connection: {}", err);
                }
            });
        }
    }
}
