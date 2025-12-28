use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use hyper_util::rt::TokioIo;

use crate::{
    cert::ClientCert,
    client::conn::{
        http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
        stream::{plain_connection, tls_connection},
    },
    errors::{ConnectionError, DeboaError},
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http1Request> {
    type Sender = Http1Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    #[hotpath::measure]
    async fn connect(
        is_secure: bool,
        host: &str,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let stream = if is_secure {
            tls_connection(host, client_cert).await
        } else {
            plain_connection(host).await
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

impl crate::client::conn::http::private::DeboaHttpConnectionSealed
    for BaseHttpConnection<Http1Request>
{
}
