use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use smol_hyper::rt::FuturesIo;

use crate::{
    cert::ClientCert,
    client::conn::{
        http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
        stream::{plain_connection, tls_connection},
    },
    errors::{ConnectionError, DeboaError},
    rt::smol::stream::SmolStream,
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http1Request> {
    type Sender = Http1Request;

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect(
        is_secure: bool,
        host: &str,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let io = if is_secure {
            tls_connection(host, client_cert).await
        } else {
            plain_connection(host).await
        };

        if let Err(e) = io {
            return Err(e);
        }

        let result = handshake(FuturesIo::new(io.unwrap())).await;

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

        Ok(BaseHttpConnection::<Http1Request> { sender })
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

impl crate::client::conn::http::private::DeboaHttpConnectionSealed
    for BaseHttpConnection<Http1Request>
{
}
