use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, client::conn::http2::handshake};
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::client::conn::http::DeboaHttpConnection;
use crate::{
    client::conn::http::{BaseHttpConnection, Http2Request},
    errors::DeboaError,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http2Request> {
    type Sender = Http2Request;

    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: Url) -> Result<BaseHttpConnection<Http2Request>, DeboaError> {
        let host = url.host().unwrap_or(Host::Domain("localhost"));
        let port = url.port().unwrap_or(80);
        let addr = format!("{host}:{port}");

        let stream = TcpStream::connect(addr).await;
        if let Err(err) = stream {
            return Err(DeboaError::Connection {
                host: host.to_string(),
                message: err.to_string(),
            });
        }

        let io = TokioIo::new(stream.unwrap());

        let result = handshake(TokioExecutor::new(), io).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection {
                host: host.to_string(),
                message: err.to_string(),
            });
        }

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::<Http2Request> { url, sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(self.url.clone(), &method, result)
    }
}
