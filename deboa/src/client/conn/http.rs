use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
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
    async fn process_response(
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
        let status_code = response.status();
        if (!status_code.is_success() && !status_code.is_redirection()) || status_code == StatusCode::TOO_MANY_REQUESTS {
            let body = response.collect().await;
            let body = body.unwrap().to_bytes().to_vec();
            return Err(DeboaError::Response {
                status_code,
                message: format!("Could not process request ({}): {}", status_code, String::from_utf8_lossy(&body)),
            });
        }

        Ok(response)
    }
}
