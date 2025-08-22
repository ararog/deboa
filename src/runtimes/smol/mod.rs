#![deny(warnings)]
#![warn(rust_2018_idioms)]

use anyhow::{bail, Result};
use async_native_tls::TlsStream;
#[cfg(feature = "http1")]
use hyper::client::conn::http1::{Connection, SendRequest};
#[cfg(feature = "http2")]
use hyper::client::conn::http2::{Connection, SendRequest};
use smol::{io, net::TcpStream, prelude::*};
use smol_hyper::rt::FuturesIo;
use std::pin::Pin;
use std::task::{Context, Poll};
use url::Url;

pub async fn get_connection(
    url: &Url,
) -> Result<(
    SendRequest<String>,
    Connection<FuturesIo<SmolStream>, String>,
)> {
    let host = url.host().expect("uri has no host");
    let io = {
        match url.scheme() {
            "http" => {
                let stream = {
                    let port = url.port().unwrap_or(80);
                    TcpStream::connect((host.to_string(), port)).await?
                };
                SmolStream::Plain(stream)
            }
            "https" => {
                // In case of HTTPS, establish a secure TLS connection first.
                let stream = {
                    let port = url.port().unwrap_or(443);
                    TcpStream::connect((host.to_string(), port)).await?
                };
                let stream = async_native_tls::connect(host.to_string(), stream).await?;
                SmolStream::Tls(stream)
            }
            scheme => bail!("unsupported scheme: {:?}", scheme),
        }
    };

    #[cfg(feature = "http1")]
    return Ok(hyper::client::conn::http1::handshake(FuturesIo::new(io)).await?);

    #[cfg(feature = "http2")]
    return Ok(hyper::client::conn::http2::handshake(FuturesIo::new(io)).await?);
}

pub enum SmolStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for SmolStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for SmolStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_close(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_flush(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}
