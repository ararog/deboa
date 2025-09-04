#![deny(warnings)]
#![warn(rust_2018_idioms)]

use hyper::client::conn::http2::handshake;
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::{
    client::conn::http2::{BaseHttp2Connection, Http2Connection},
    errors::DeboaError,
};

pub struct DeboaHttp2Connection;

#[async_trait::async_trait]
impl Http2Connection for DeboaHttp2Connection {
    async fn connect(url: Url) -> Result<BaseHttp2Connection, DeboaError> {
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

        Ok(BaseHttp2Connection::new(url, sender))
    }
}
