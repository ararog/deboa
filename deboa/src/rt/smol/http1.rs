use std::marker::PhantomData;

use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use smol_hyper::rt::FuturesIo;

use crate::{
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection, ConnectionConfig},
    request::Http1Request,
    rt::smol::tls::{plain_connection, tls_connection},
    Result,
};

impl DeboaTcpConnection for BaseHttpConnection<Http1Request, Full<Bytes>, Incoming> {
    type Sender = Http1Request;
    type ReqBody = Full<Bytes>;
    type ResBody = Incoming;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect<'a>(
        config: &ConnectionConfig<'a>,
    ) -> Result<BaseHttpConnection<Http1Request, Full<Bytes>, Incoming>> {
        let stream = if config.is_secure() {
            tls_connection(
                config.host(),
                config.port(),
                config.identity(),
                config.certificate(),
                config.skip_cert_verification(),
                None,
            )
            .await
        } else {
            plain_connection(config.host(), config.port()).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(FuturesIo::new(stream.unwrap())).await;

        let (sender, conn) = result.unwrap();

        smol::spawn(async move {
            match conn
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::<Http1Request, Full<Bytes>, Incoming> {
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
    for BaseHttpConnection<Http1Request, Full<Bytes>, Incoming>
{
}
