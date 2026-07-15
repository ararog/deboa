use crate::{
    alpn,
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{
        stream::{plain_connection, tls_connection},
        BaseHttpConnection, Http2Connection,
    },
    Result,
};
use cyper_core::CompioExecutor;
use deboa::{
    conn::{ConnectionConfig, HttpConnection, ProtoConnection, SendRequest},
    request::Http2Request,
};
use http::version::Version;
use hyper::client::conn::http2::handshake;
use hyper_body_utils::HttpBody;

impl HttpConnection for Http2Connection {
    type Sender = SendRequest<Http2Request, HttpBody>;
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
    fn protocol(&self) -> Version {
        Version::HTTP_2
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

        let result = handshake(CompioExecutor, stream.unwrap()).await;

        let (sender, conn) = result.unwrap();

        compio::runtime::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::new(SendRequest::new(sender)))
    }
}
