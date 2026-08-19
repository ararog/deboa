use crate::{
    client::http::conn::{BaseHttpConnection, Http1Connection},
    rt::stream::SmolStream,
};
use deboa::{
    conn::{HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError},
    request::Http1Request,
    Result,
};
use http::version::Version;
use hyper::client::conn::http1::handshake;
use smol_hyper::rt::FuturesIo;

impl HttpConnection for Http1Connection {
    type Sender = Http1Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http1Connection {
    type Connection = Http1Connection;
    type RuntimeStream = SmolStream;

    #[inline]
    fn protocol_version(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect(stream: Self::RuntimeStream) -> Result<Self::Connection> {
        let (sender, conn) = handshake(FuturesIo::new(stream))
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Handshake { message: e.to_string() })
            })?;

        smol::spawn(async move {
            match conn
                .with_upgrades()
                .await
            {
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
