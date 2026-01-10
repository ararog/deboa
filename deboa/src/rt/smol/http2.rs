use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use smol_hyper::rt::FuturesIo;

use crate::{
    cert::{Certificate, Identity},
    client::conn::{tcp::DeboaTcpConnection, BaseHttpConnection},
    request::Http2Request,
    rt::smol::{
        executor::SmolExecutor,
        tls::{plain_connection, tls_connection},
    },
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
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let io = if is_secure {
            tls_connection(host, port, identity, certificate, skip_cert_verification, Some("h2"))
                .await
        } else {
            plain_connection(host, port).await
        };

        if let Err(e) = io {
            return Err(e);
        }

        let result = handshake(SmolExecutor::new(), FuturesIo::new(io.unwrap())).await;

        let (sender, conn) = result.unwrap();

        smol::spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::<Self::Sender> { sender })
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
