#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use crate::{alpn, client::http::conn::stream::tls::tls_connection};
use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{stream::plain::plain_connection, BaseHttpConnection},
};
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    request::Http1Request,
    Result,
};
use http::version::Version;
use hyper::client::conn::http1::handshake;
use hyper_body_utils::HttpBody;
use hyper_util::rt::TokioIo;

impl HttpConnection for BaseHttpConnection<Http1Request, HttpBody, HttpBody> {
    type Sender = Http1Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for BaseHttpConnection<Http1Request, HttpBody, HttpBody> {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = BaseHttpConnection<Http1Request, HttpBody, HttpBody>;
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<Self::Connection> {
        #[cfg(any(feature = "rust-tls", feature = "native-tls"))]
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

        #[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
        let stream = plain_connection(config.host(), config.port()).await;

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(TokioIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::new(sender))
    }
}
