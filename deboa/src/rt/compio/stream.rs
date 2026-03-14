use compio_tls::TlsStream;

use std::pin::Pin;

use compio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

pub enum CompioStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by native TLS.
    #[cfg(feature = "compio-native-tls")]
    Tls(TlsStream<TcpStream>),

    /// A TCP connection secured by rustls.
    #[cfg(feature = "compio-rust-tls")]
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for CompioStream {
    async fn read<B: compio::buf::IoBufMut>(&mut self, buf: B) -> compio::BufResult<usize, B> {
        match &mut *self {
            CompioStream::Plain(stream) => {
                Pin::new(stream)
                    .read(buf)
                    .await
            }
            CompioStream::Tls(stream) => {
                Pin::new(stream)
                    .read(buf)
                    .await
            }
        }
    }

    async fn read_vectored<V: compio::buf::IoVectoredBufMut>(
        &mut self,
        buf: V,
    ) -> compio::BufResult<usize, V> {
        match &mut *self {
            CompioStream::Plain(stream) => {
                stream
                    .read_vectored(buf)
                    .await
            }
            CompioStream::Tls(stream) => {
                stream
                    .read_vectored(buf)
                    .await
            }
        }
    }
}

impl AsyncWrite for CompioStream {
    async fn write<T: compio::buf::IoBuf>(&mut self, buf: T) -> compio::BufResult<usize, T> {
        match self {
            CompioStream::Plain(stream) => {
                Pin::new(stream)
                    .write(buf)
                    .await
            }
            CompioStream::Tls(stream) => {
                Pin::new(stream)
                    .write(buf)
                    .await
            }
        }
    }

    async fn write_vectored<T: compio::buf::IoVectoredBuf>(
        &mut self,
        buf: T,
    ) -> compio::BufResult<usize, T> {
        match self {
            CompioStream::Plain(stream) => {
                Pin::new(stream)
                    .write_vectored(buf)
                    .await
            }
            CompioStream::Tls(stream) => {
                Pin::new(stream)
                    .write_vectored(buf)
                    .await
            }
        }
    }

    async fn flush(&mut self) -> std::io::Result<()> {
        match self {
            CompioStream::Plain(stream) => {
                Pin::new(stream)
                    .flush()
                    .await
            }
            CompioStream::Tls(stream) => {
                Pin::new(stream)
                    .flush()
                    .await
            }
        }
    }

    async fn shutdown(&mut self) -> std::io::Result<()> {
        match self {
            CompioStream::Plain(stream) => {
                Pin::new(stream)
                    .shutdown()
                    .await
            }
            CompioStream::Tls(stream) => {
                Pin::new(stream)
                    .shutdown()
                    .await
            }
        }
    }
}
