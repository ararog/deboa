#[cfg(feature = "native-tls")]
use async_native_tls::TlsStream;

#[cfg(feature = "rust-tls")]
use futures_rustls::client::TlsStream;

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use smol::{
    io::{self, AsyncRead, AsyncWrite},
    net::TcpStream,
};

/// A stream that can be either plain TCP or TLS-secured.
pub enum SmolStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by native TLS.
    #[cfg(feature = "native-tls")]
    Tls(TlsStream<TcpStream>),

    /// A TCP connection secured by rustls.
    #[cfg(feature = "rust-tls")]
    Tls(Box<TlsStream<TcpStream>>),
}

impl AsyncRead for SmolStream {
    /// Polls for reading data from the stream.
    ///
    /// # Arguments
    ///
    /// * `cx` - The context to use for polling.
    /// * `buf` - The buffer to read data into.
    ///
    /// # Returns
    ///
    /// * `Poll<io::Result<usize>>` - The result of the read operation.
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            #[cfg(any(feature = "native-tls", feature = "rust-tls"))]
            SmolStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for SmolStream {
    /// Polls for writing data to the stream.
    ///
    /// # Arguments
    ///
    /// * `cx` - The context to use for polling.
    /// * `buf` - The buffer to write data from.
    ///
    /// # Returns
    ///
    /// * `Poll<io::Result<usize>>` - The result of the write operation.
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            #[cfg(any(feature = "native-tls", feature = "rust-tls"))]
            SmolStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_close(cx),
            #[cfg(any(feature = "native-tls", feature = "rust-tls"))]
            SmolStream::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_flush(cx),
            #[cfg(any(feature = "native-tls", feature = "rust-tls"))]
            SmolStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}
