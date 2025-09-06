use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Incoming, Request, Response};
use url::Url;

use crate::errors::DeboaError;

pub struct BaseHttpConnection<T> {
    pub(crate) url: Url,
    pub(crate) sender: T,
}

pub type Http1Request = hyper::client::conn::http1::SendRequest<Full<Bytes>>;
pub type Http2Request = hyper::client::conn::http2::SendRequest<Full<Bytes>>;

#[async_trait::async_trait]
pub trait DeboaHttpConnection<T> {
    async fn connect(url: Url) -> Result<BaseHttpConnection<T>, DeboaError>;
    fn url(&self) -> &Url;
    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError>;
}
