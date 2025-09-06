use bytes::Bytes;
use http::StatusCode;
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
    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    ///
    /// # Returns
    ///
    /// * `Result<BaseHttpConnection<T>, DeboaError>` - The connection or error.
    ///
    async fn connect(url: Url) -> Result<BaseHttpConnection<T>, DeboaError>;

    /// Get connection url.
    ///
    /// # Returns
    ///
    /// * `&Url` - The connection url.
    ///
    fn url(&self) -> &Url;

    /// Send a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>, DeboaError>` - The response or error.
    ///
    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError>;

    /// Process a response.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    /// * `method` - The method to use.
    /// * `response` - The response to process.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>, DeboaError>` - The response or error.
    ///
    fn process_response(
        &mut self,
        url: Url,
        method: &str,
        response: Result<Response<Incoming>, hyper::Error>,
    ) -> Result<Response<Incoming>, DeboaError> {
        if let Err(err) = response {
            return Err(DeboaError::Request {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let response = response.unwrap();
        if !response.status().is_success() || response.status() == StatusCode::TOO_MANY_REQUESTS {
            return Err(DeboaError::Response {
                status_code: response.status(),
                message: response.status().to_string(),
            });
        }

        Ok(response)
    }
}
