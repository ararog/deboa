use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use hyper_util::rt::TokioIo;

use crate::{
    cert::Identity,
    client::conn::{
        stream::{plain_connection, tls_connection},
        tcp::DeboaTcpConnection,
        BaseHttpConnection,
    },
    errors::{ConnectionError, DeboaError},
    request::Http1Request,
    Result,
};

#[async_trait]
impl DeboaTcpConnection for BaseHttpConnection<Http1Request> {
    type Sender = Http1Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    #[hotpath::measure]
    async fn connect(
        is_secure: bool,
        host: &str,
        port: u16,
        client_cert: &Option<Identity>,
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let stream = if is_secure {
            tls_connection(host, port, client_cert).await
        } else {
            plain_connection(host, port).await
        };

        if let Err(e) = stream {
            return Err(e);
        }

        let result = handshake(TokioIo::new(stream.unwrap())).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection(ConnectionError::Handshake {
                host: host.to_string(),
                message: err.to_string(),
            }));
        }

        let (sender, conn) = result.unwrap();

        tokio::spawn(async move {
            match conn
                .with_upgrades()
                .await
            {
                Ok(_) => (),
                Err(_err) => {}
            };
        });

        Ok(BaseHttpConnection::<Self::Sender> { sender })
    }

    #[hotpath::measure]
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
    for BaseHttpConnection<Http1Request>
{
}
