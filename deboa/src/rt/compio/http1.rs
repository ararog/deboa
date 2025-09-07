use std::str::FromStr;

use async_trait::async_trait;
use bytes::Bytes;
use compio::runtime::spawn;
use cyper_core::{HttpStream, TlsBackend};
use http::Uri;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, client::conn::http1::handshake};
use url::Url;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::DeboaError,
};

#[async_trait]
impl DeboaHttpConnection<Http1Request> for BaseHttpConnection<Http1Request> {
    fn url(&self) -> &Url {
        &self.url
    }

    async fn connect(url: Url) -> Result<BaseHttpConnection<Http1Request>, DeboaError> {
        let uri = Uri::from_str(url.as_str()).unwrap();
        let stream = HttpStream::connect(uri, TlsBackend::default()).await;
        if let Err(err) = stream {
            return Err(DeboaError::Connection {
                host: url.host().unwrap().to_string(),
                message: err.to_string(),
            });
        }

        let stream = stream.unwrap();
        let result = handshake(stream).await;

        if let Err(err) = result {
            return Err(DeboaError::Connection {
                host: url.host().unwrap().to_string(),
                message: err.to_string(),
            });
        }

        let (sender, conn) = result.unwrap();

        spawn(async move {
            match conn.await {
                Ok(_) => (),
                Err(_err) => {}
            };
        })
        .detach();

        Ok(BaseHttpConnection::<Http1Request> { url, sender })
    }

    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError> {
        let method = request.method().to_string();
        let result = self.sender.send_request(request).await;

        self.process_response(self.url.clone(), &method, result)
    }
}
