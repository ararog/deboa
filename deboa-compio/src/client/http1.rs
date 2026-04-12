use std::marker::PhantomData;

use http::version::Version;
use hyper::{client::conn::http1::handshake, Request, Response};

use rt_gate::spawn_worker;

#[cfg(all(feature = "smol-rt", any(feature = "smol-rust-tls", feature = "smol-native-tls")))]
use crate::rt::smol::tls::{plain_connection, tls_connection};
#[cfg(feature = "smol-rt")]
use smol_hyper::rt::FuturesIo;

#[cfg(all(
    feature = "tokio-rt",
    any(feature = "tokio-rust-tls", feature = "tokio-native-tls")
))]
use crate::rt::tokio::tls::{plain_connection, tls_connection};
#[cfg(feature = "tokio-rt")]
use hyper_util::rt::TokioIo;

use hyper_body_utils::HttpBody;

use crate::{
    alpn,
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection, ConnectionConfig},
    request::Http1Request,
    Result,
};

#[cfg(feature = "smol-rt")]
type DeboaIo<T> = FuturesIo<T>;

#[cfg(feature = "tokio-rt")]
type DeboaIo<T> = TokioIo<T>;

impl DeboaTcpConnection for BaseHttpConnection<Http1Request, HttpBody, HttpBody> {
    type Sender = Http1Request;
    type ReqBody = HttpBody;
    type ResBody = HttpBody;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a>,
    ) -> Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>> {
        let stream = if config.is_secure() {
            tls_connection(
                config.host(),
                config.port(),
                config.identity(),
                config.certificate(),
                config.skip_cert_verification(),
                alpn(),
            )
            .await
        } else {
            plain_connection(config.host(), config.port()).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(DeboaIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        spawn_worker(async move {
            match conn
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

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
    for BaseHttpConnection<Http1Request, HttpBody, HttpBody>
{
}
