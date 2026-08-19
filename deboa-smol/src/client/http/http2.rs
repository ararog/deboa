use crate::{
    client::http::conn::{BaseHttpConnection, Http2Connection},
    rt::{executor::SmolExecutor, stream::SmolStream},
};
use deboa::{
    conn::{HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError},
    request::Http2Request,
    Result,
};
use http::version::Version;
use hyper::client::conn::http2::handshake;
use smol_hyper::rt::FuturesIo;

impl HttpConnection for Http2Connection {
    type Sender = Http2Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http2Connection {
    type Connection = Http2Connection;
    type RuntimeStream = SmolStream;

    #[inline]
    fn protocol_version(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect(stream: Self::RuntimeStream) -> Result<Self::Connection> {
        let (sender, conn) = handshake(SmolExecutor::new(), FuturesIo::new(stream))
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Handshake { message: e.to_string() })
            })?;

        smol::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(err) => {
                    log::error!("Error: {:#}", err)
                }
            };
        })
        .detach();

        Ok(BaseHttpConnection::new(sender))
    }
}
