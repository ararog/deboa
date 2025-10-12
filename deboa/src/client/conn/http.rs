use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode, Version};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use url::Url;

use crate::{cert::ClientCert, errors::DeboaError, MAX_ERROR_MESSAGE_SIZE, Result};

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
    pub(crate) url: Arc<Url>,
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
    /// * `Result<BaseHttpConnection<Self::Sender>>` - The connection or error.
    ///
    async fn connect(
        url: Arc<Url>,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>>;

    /// Get connection url.
    ///
    /// # Returns
    ///
    /// * `&Url` - The connection url.
    ///
    fn url(&self) -> &Url;

    /// Get connection protocol.
    ///
    /// # Returns
    ///
    /// * `Version` - The connection protocol.
    ///
    fn protocol(&self) -> Version;

    /// Send a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response or error.
    ///
    async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>>;

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
    /// * `Result<Response<Incoming>>` - The response or error.
    ///
    async fn process_response(
        &self,
        url: &Url,
        method: &str,
        response: std::result::Result<Response<Incoming>, hyper::Error>,
    ) -> Result<Response<Incoming>> {
        if let Err(err) = response {
            return Err(DeboaError::Request {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let response = response.unwrap();
        let status_code = response.status();
        if (!status_code.is_success() && !status_code.is_redirection())
            || status_code == StatusCode::TOO_MANY_REQUESTS
        {
            let mut body = response.into_body();
            let mut error_message = Vec::new();
            let mut downloaded = 0;
            while let Some(chunk) = body.frame().await {
                if let Ok(frame) = chunk {
                    if let Some(data) = frame.data_ref() {
                        if downloaded + data.len() > MAX_ERROR_MESSAGE_SIZE {
                            break;
                        }
                        error_message.extend_from_slice(data);
                        downloaded += data.len();
                    }
                }
            }
            return Err(DeboaError::Response {
                status_code,
                message: format!(
                    "Could not process request ({}): {}",
                    status_code,
                    String::from_utf8_lossy(&error_message)
                ),
            });
        }

        Ok(response)
    }
}
