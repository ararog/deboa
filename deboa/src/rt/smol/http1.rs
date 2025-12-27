use std::sync::Arc;

use async_native_tls::{Identity, TlsConnector};
use async_trait::async_trait;
use bytes::Bytes;
use http::version::Version;
use http_body_util::Full;
use hyper::{body::Incoming, client::conn::http1::handshake, Request, Response};
use smol::net::TcpStream;
use smol_hyper::rt::FuturesIo;
use url::Url;

use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::{ConnectionError, DeboaError},
    rt::smol::stream::SmolStream,
    Result,
};

#[async_trait]
impl DeboaHttpConnection for BaseHttpConnection<Http1Request> {
    type Sender = Http1Request;

    #[inline]
    fn url(&self) -> &Url {
        &self.url
    }

    #[inline]
    fn protocol(&self) -> Version {
        Version::HTTP_11
    }

    async fn connect(
        url: Arc<Url>,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>> {
        let host = url
            .host_str()
            .unwrap_or("localhost");
        let io = {
            match url.scheme() {
                "ws" | "http" => Self::plain_connection(Arc::clone(&url)).await,
                "wss" | "https" => Self::tls_connection(Arc::clone(&url), client_cert).await,
                scheme => {
                    return Err(DeboaError::Connection(ConnectionError::UnsupportedScheme {
                        message: format!("unsupported scheme: {scheme:?}"),
                    }));
                }
            }
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

        Ok(BaseHttpConnection::<Http1Request> { url, sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>> {
        let method = request
            .method()
            .to_string();
        let result = self
            .sender
            .send_request(request)
            .await;

        self.process_response(&self.url, &method, result)
            .await
    }
}

impl crate::client::conn::http::private::DeboaHttpConnectionSealed
    for BaseHttpConnection<Http1Request>
{
}
