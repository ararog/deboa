use crate::client::http::conn::{BaseHttpConnection, Http3Connection};
use compio_quic::Connection;
use deboa::{
    conn::{HttpConnection, ProtoConnection},
    errors::{ConnectionError, DeboaError},
    Result,
};
use deboa_h3::compio::{Http3Request, SendRequest};
use futures::future;
use http::version::Version;

impl HttpConnection for Http3Connection {
    type Sender = Http3Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http3Connection {
    type Connection = Http3Connection;
    type RuntimeStream = Connection;

    #[inline]
    fn protocol_version(&self) -> Version {
        Version::HTTP_3
    }

    async fn connect(stream: Self::RuntimeStream) -> Result<Self::Connection> {
        let (mut conn, sender) = compio_quic::h3::client::new(stream)
            .await
            .map_err(|e| DeboaError::Connection(ConnectionError::Udp { message: e.to_string() }))?;

        compio::runtime::spawn(async move {
            future::poll_fn(|cx| conn.poll_close(cx)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .detach();

        Ok(BaseHttpConnection::new(SendRequest::new(sender)))
    }
}
