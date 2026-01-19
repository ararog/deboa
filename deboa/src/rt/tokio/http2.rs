use std::marker::PhantomData;

use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Response};
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;

use crate::client::conn::ConnectionConfig;
use crate::{
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection},
    errors::{ConnectionError, DeboaError},
    request::Http2Request,
    rt::tokio::tls::{plain_connection, tls_connection},
    Result,
};

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

        let result = handshake(TokioExecutor::new(), TokioIo::new(stream.unwrap())).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection(ConnectionError::Handshake {
                host: config
                    .host()
                    .to_string(),
                message: err.to_string(),
            }));
        }

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::<Http2Request, Self::ReqBody, Self::ResBody> {
            sender,
            req_body: PhantomData,
            res_body: PhantomData,
        })
    }

    async fn send_request(
        &mut self,
        request: http::Request<Full<Bytes>>,
    ) -> Result<Response<Incoming>> {
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
