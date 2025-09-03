#![deny(warnings)]
#![warn(rust_2018_idioms)]

use bytes::Bytes;
#[cfg(feature = "http1")]
use hyper::client::conn::http1::{Connection, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::errors::DeboaError;

pub async fn get_connection(
    url: &Url,
) -> Result<
    (
        SendRequest<http_body_util::Full<Bytes>>,
        Connection<TokioIo<TcpStream>, http_body_util::Full<Bytes>>,
    ),
    DeboaError,
> {
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

    let result = hyper::client::conn::http1::handshake(io).await;

    match result {
        Ok(conn) => Ok(conn),
        Err(err) => Err(DeboaError::Connection {
            host: host.to_string(),
            message: err.to_string(),
        }),
    }
}
