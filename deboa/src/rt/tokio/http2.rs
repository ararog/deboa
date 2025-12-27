use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http2::handshake, Request, Response};
use hyper_util::rt::TokioExecutor;
use hyper_util::rt::TokioIo;

use crate::client::conn::http::DeboaHttpConnection;
use crate::errors::ConnectionError;
use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, Http2Request},
    errors::DeboaError,
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http2Request> {
    type Sender = Http2Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_2
    }

    #[hotpath::measure]
    async fn connect(
        is_secure: bool,
        host: &str,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Http2Request>> {
        let stream = if is_secure {
            Self::tls_connection(&host, client_cert).await
        } else {
            Self::plain_connection(&host).await
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

impl crate::client::conn::http::private::DeboaHttpConnectionSealed
    for BaseHttpConnection<Http2Request>
{
}
