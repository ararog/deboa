use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio_native_tls::TlsStream;

use tokio::{
    io::{self, AsyncRead, AsyncWrite},
    net::TcpStream,
};

#[allow(clippy::large_enum_variant)]
pub enum TokioStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for TokioStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::result::Result<(), std::io::Error>> {
        match &mut *self {
            TokioStream::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            TokioStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TokioStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            TokioStream::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            TokioStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        match &mut *self {
            TokioStream::Plain(stream) => Pin::new(stream).poll_shutdown(cx),
            TokioStream::Tls(stream) => Pin::new(stream).poll_shutdown(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            TokioStream::Plain(stream) => Pin::new(stream).poll_flush(cx),
            TokioStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}
