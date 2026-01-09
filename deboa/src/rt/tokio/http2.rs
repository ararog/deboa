use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;

use crate::cert::Certificate;
use crate::{
    cert::Identity,
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection},
    errors::{ConnectionError, DeboaError},
    request::Http2Request,
    rt::tokio::tls::{plain_connection, tls_connection},
    Result,
};

#[async_trait]
impl DeboaTcpConnection for BaseHttpConnection<Http2Request> {
    type Sender = Http2Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_2
    }

    async fn connect(
        is_secure: bool,
        host: &str,
        port: u16,
        identity: &Option<Identity>,
        certificate: &Option<Certificate>,
        skip_cert_verification: bool,
    ) -> Result<BaseHttpConnection<Http2Request>> {
        let stream = if is_secure {
            tls_connection(host, port, identity, certificate, skip_cert_verification, Some("h2"))
                .await
        } else {
            plain_connection(host, port).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(TokioExecutor::new(), TokioIo::new(stream.unwrap())).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection(ConnectionError::Handshake {
                host: host.to_string(),
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

        Ok(BaseHttpConnection::<Http2Request> { sender })
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
    for BaseHttpConnection<Http2Request>
{
}
