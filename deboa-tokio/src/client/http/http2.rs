use crate::{
    client::http::conn::{BaseHttpConnection, Http2Connection},
    rt::stream::TokioStream,
};
use deboa::{
    conn::{HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError},
    request::Http2Request,
    Result,
};
use http::version::Version;
use hyper::client::conn::http2::handshake;
use hyper_util::rt::{TokioExecutor, TokioIo};

impl HttpConnection for Http2Connection {
    type Sender = Http2Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http2Connection {
    type Connection = Http2Connection;
    type RuntimeStream = TokioStream;

    #[inline]
    fn protocol_version(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect(stream: Self::RuntimeStream) -> Result<Self::Connection> {
        let (sender, conn) = handshake(TokioExecutor::new(), TokioIo::new(stream))
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Handshake { message: e.to_string() })
            })?;

        tokio::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(err) => {
                    println!("Error: {:#}", err)
                }
            };
        });

        Ok(BaseHttpConnection::new(sender))
    }
}
