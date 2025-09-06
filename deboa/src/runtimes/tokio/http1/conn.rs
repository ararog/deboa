#![deny(warnings)]
#![warn(rust_2018_idioms)]

use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::DeboaError,
};

#[async_trait::async_trait]
impl DeboaHttpConnection<Http1Request> for BaseHttpConnection<Http1Request> {
    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: Url) -> Result<BaseHttpConnection<Http1Request>, DeboaError> {
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

        let result = handshake(io).await;

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

        Ok(BaseHttpConnection::<Http1Request> { url, sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError> {
        let result = self.sender.send_request(request).await;
        if let Err(err) = result {
            return Err(DeboaError::Response {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: err.to_string(),
            });
        }

        let response = result.unwrap();
        if !response.status().is_success() || response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(DeboaError::Response {
                status_code: response.status(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }
}
