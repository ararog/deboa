use std::marker::PhantomData;
use std::result;

use deboa::request::Http2Request;
use http::version::Version;
use hyper::{client::conn::http2::handshake, Request, Response};

use hyper_body_utils::HttpBody;

use crate::rt::executor::CompioExecutor;
use crate::rt::tls::{plain_connection, tls_connection};
use crate::rt::CompioIo;


use crate::{
    alpn,
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection, ConnectionConfig},
    Result,
};

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
