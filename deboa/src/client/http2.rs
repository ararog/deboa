use std::marker::PhantomData;

use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};

use rt_gate::spawn_worker;

#[cfg(feature = "smol-rt")]
use crate::rt::smol::executor::SmolExecutor;
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
use hyper_util::rt::TokioExecutor;
#[cfg(feature = "tokio-rt")]
use hyper_util::rt::TokioIo;

use crate::{
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection, ConnectionConfig},
    request::Http2Request,
    Result,
};

#[cfg(feature = "smol-rt")]
type DeboaIo<T> = FuturesIo<T>;
#[cfg(feature = "smol-rt")]
type DeboaExecutor = SmolExecutor;

#[cfg(feature = "tokio-rt")]
type DeboaIo<T> = TokioIo<T>;
#[cfg(feature = "tokio-rt")]
type DeboaExecutor = TokioExecutor;

impl DeboaTcpConnection for BaseHttpConnection<Http2Request, Full<Bytes>, Incoming> {
    type Sender = Http2Request;
    type ReqBody = Full<Bytes>;
    type ResBody = Incoming;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_2
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
                Some("h2"),
            )
            .await
        } else {
            plain_connection(config.host(), config.port()).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(DeboaExecutor::new(), DeboaIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        spawn_worker(async move {
            match conn.await {
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

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
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
    for BaseHttpConnection<Http2Request, Full<Bytes>, Incoming>
{
}
