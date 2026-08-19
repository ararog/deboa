#[cfg(feature = "rust-tls")]
use crate::alpn;
#[cfg(feature = "rust-tls")]
use crate::client::http::conn::stream::tls_connection;
use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{stream::plain_connection, BaseHttpConnection, Http2Connection},
    rt::executor::GlommioExecutor,
};
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection},
    request::Http2Request,
    Result,
};
use http::version::Version;
use hyper::client::conn::http2::handshake;
use hyper_body_utils::HttpBody;
use smol_hyper::rt::FuturesIo;

impl HttpConnection for Http2Connection {
    type Sender = Http2Request;
    fn sender(&mut self) -> &mut Self::Sender {
        &mut self.sender
    }
}

impl ProtoConnection for Http2Connection {
    type ReqBody = HttpBody;
    type ResBody = HttpBody;
    type Connection = Http2Connection;
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;

    #[inline]
    fn protocol_version(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<Self::Connection> {
        #[cfg(feature = "rust-tls")]
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

        #[cfg(not(feature = "rust-tls"))]
        let stream = plain_connection(*config.ip(), config.host(), config.port()).await;

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(GlommioExecutor::new(), FuturesIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        glommio::spawn_local(async move {
            match conn.await {
                Ok(_) => (),
                Err(err) => {
                    log::debug!("http2 connection ended: {err:#}");
                }
            };
        })
        .detach();

        Ok(BaseHttpConnection::new(sender))
    }
}
