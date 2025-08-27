#![deny(warnings)]
#![warn(rust_2018_idioms)]

use bytes::Bytes;
#[cfg(feature = "http1")]
use hyper::client::conn::http1::{Connection, SendRequest};
#[cfg(feature = "http2")]
use hyper::client::conn::http2::{Connection, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

use crate::DeboaError;

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
        return Err(DeboaError::ConnectionError {
            host: host.to_string(),
            message: err.to_string(),
        });
    }

    let io = TokioIo::new(stream.unwrap());

    #[cfg(feature = "http1")]
    let result = hyper::client::conn::http1::handshake(io).await;
    
    #[cfg(feature = "http1")]
    match result {
        Ok(conn) => {
            return Ok(conn);
        },
        Err(err) => {
            return Err(DeboaError::ConnectionError {
                host: host.to_string(),
                message: err.to_string(),
            });
        },
    }

    #[cfg(feature = "http2")]
    let result = hyper::client::conn::http2::handshake(io).await;
    #[cfg(feature = "http2")]
    if let Err(err) = result {
        return Err(DeboaError::ConnectionError {
            host: host.to_string(),
            message: err.to_string(),
        });
    }
    #[cfg(feature = "http2")]
    return Ok(result.unwrap());
}
