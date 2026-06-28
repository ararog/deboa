#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use crate::alpn;
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use crate::client::conn::stream::tls::tls_connection;
use crate::{
    client::conn::{
        stream::plain::plain_connection, tcp::DeboaTcpConnection, BaseHttpConnection,
        ConnectionConfig,
    },
    rt::executor::SmolExecutor,
    Result,
};
use deboa::request::Http2Request;
use http::version::Version;
use hyper::{client::conn::http2::handshake, Request, Response};
use hyper_body_utils::HttpBody;
use smol_hyper::rt::FuturesIo;
use std::marker::PhantomData;

impl DeboaTcpConnection for BaseHttpConnection<Http2Request, HttpBody, HttpBody> {
    type Sender = Http2Request;
    type ReqBody = HttpBody;
    type ResBody = HttpBody;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a>,
    ) -> Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>> {
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
        let stream = plain_connection(*config.ip(), config.host(), config.port()).await;

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(SmolExecutor::new(), FuturesIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        smol::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::<Self::Sender, Self::ReqBody, Self::ResBody> {
            sender,
            req_body: PhantomData,
            res_body: PhantomData,
        })
    }

    async fn send_request(
        &mut self,
        request: Request<Self::ReqBody>,
    ) -> Result<Response<Self::ResBody>> {
        let method = request
            .method()
            .to_string();

        let result = self
            .sender
            .send_request(request)
            .await;

        self.process_response(&method, result)
            .await
    }
}

impl crate::client::conn::tcp::private::DeboaTcpConnectionSealed
    for BaseHttpConnection<Http2Request, HttpBody, HttpBody>
{
}
