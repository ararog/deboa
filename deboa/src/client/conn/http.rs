use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming};
use url::Url;

use crate::errors::DeboaError;

#[derive(Debug)]
/// Enum that represents the connection type.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 connection.
/// * `Http2` - The HTTP/2 connection.
pub enum DeboaConnection {
    #[cfg(feature = "http1")]
    Http1(Box<BaseHttpConnection<Http1Request>>),
    #[cfg(feature = "http2")]
    Http2(Box<BaseHttpConnection<Http2Request>>),
}

#[derive(Debug, Clone)]
/// Struct that represents the connection.
///
/// # Fields
///
/// * `url` - The url to connect.
/// * `sender` - The sender to use.
pub struct BaseHttpConnection<T> {
    pub(crate) url: Url,
    pub(crate) sender: T,
}

#[cfg(feature = "http1")]
pub type Http1Request = hyper::client::conn::http1::SendRequest<Full<Bytes>>;
#[cfg(feature = "http2")]
pub type Http2Request = hyper::client::conn::http2::SendRequest<Full<Bytes>>;

#[async_trait]
/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaHttpConnection {
    type Sender;

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    ///
    /// # Returns
    ///
    /// * `Result<BaseHttpConnection<Self::Sender>, DeboaError>` - The connection or error.
    ///
    async fn connect(url: &Url) -> Result<BaseHttpConnection<Self::Sender>, DeboaError>;

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
        &self,
        url: &Url,
        method: &str,
        response: Result<Response<Incoming>, hyper::Error>,
    ) -> Result<Response<Incoming>, DeboaError> {
        if let Err(err) = response {
            return Err(DeboaError::Request {
                url: url.to_string(),
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
