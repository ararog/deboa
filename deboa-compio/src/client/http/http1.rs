#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use crate::alpn;
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use crate::client::http::conn::stream::tls_connection;
use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{stream::plain_connection, BaseHttpConnection, Http1Connection},
    Result,
};
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    request::Http1Request,
};
use http::version::Version;
use hyper::client::conn::http1::handshake;
use hyper_body_utils::HttpBody;

impl HttpConnection for Http1Connection {
    type Sender = Http1Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http1Connection {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = Http1Connection;
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<Self::Connection> {
        let stream = if config.is_secure() {
            tls_connection(
                *config.ip(),
                config.host(),
                config.port(),
                config.identity(),
                config.certificate(),
                config.skip_cert_verification(),
                alpn(),
            )
            .await
        } else {
            plain_connection(*config.ip(), config.host(), config.port()).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(stream.unwrap()).await;

        let (sender, conn) = result.unwrap();

        compio::runtime::spawn(async move {
            match conn
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::new(sender))
    }
}
