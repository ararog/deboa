#![deny(warnings)]
#![warn(rust_2018_idioms)]

use anyhow::Result;
#[cfg(feature = "http1")]
use hyper::client::conn::http1::{Connection, SendRequest};
#[cfg(feature = "http2")]
use hyper::client::conn::http2::{Connection, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use url::{Host, Url};

pub async fn get_connection(
    url: &Url,
) -> Result<(SendRequest<String>, Connection<TokioIo<TcpStream>, String>)> {
    let host = url.host().unwrap_or(Host::Domain("localhost"));
    let port = url.port().unwrap_or(80);
    let addr = format!("{host}:{port}");

    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);
    
    #[cfg(feature = "http1")]
    return Ok(hyper::client::conn::http1::handshake(io).await?);

    #[cfg(feature = "http2")]
    return Ok(hyper::client::conn::http2::handshake(io).await?);
}
