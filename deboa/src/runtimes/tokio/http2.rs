#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::client::conn::http2::handshake;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::{
    client::io::http::{BaseHttpConnection, HttpConnection},
    errors::DeboaError,
};

pub struct Http2Connection;

#[async_trait::async_trait]
impl HttpConnection for Http2Connection {
    async fn connect(url: Url) -> Result<BaseHttpConnection, DeboaError> {
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
                Err(_err) => {
                    // return Err(DeboaError::ConnectionError {
                    //     host: url.to_string(),
                    //     message: err.to_string(),
                    // });
                }
            };
        });

        Ok(BaseHttpConnection::new(url, sender))
    }
}
