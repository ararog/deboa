use std::future::Future;

use http::{Request, Response, StatusCode, Version};
use http_body::Body;
use http_body_util::BodyExt;
use hyper::body::Incoming;

use crate::{
    client::conn::{BaseHttpConnection, ConnectionConfig},
    errors::{DeboaError, RequestError, ResponseError},
    Result, MAX_ERROR_MESSAGE_SIZE,
};

/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaTcpConnection: private::DeboaTcpConnectionSealed {
    type Sender;
    type ReqBody: Body + Unpin;
    type ResBody: Body + Unpin;

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `is_secure` - Whether the connection is secure.
    /// * `host` - The host to connect.
    /// * `port` - The port to connect.
    /// * `identity` - The identity to use.
    /// * `certificate` - The certificate to use.
    /// * `skip_cert_verification` - Whether to skip certificate verification.
    ///
    /// # Errors
    ///
    /// * `DeboaError` - If the connection fails.
    ///
    /// # Returns
    ///
    /// * `Result<BaseHttpConnection<Self::Sender>>` - The connection or error.
    ///
    fn connect(
        config: &ConnectionConfig,
    ) -> impl Future<Output = Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>>>;

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
    /// * `Result<Response<Self::ResBody>>` - The response or error.
    ///
    fn send_request(
        &mut self,
        request: Request<Self::ReqBody>,
    ) -> impl Future<Output = Result<Response<Self::ResBody>>>;

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
    fn process_response(
        &self,
        method: &str,
        response: std::result::Result<Response<Incoming>, hyper::Error>,
    ) -> impl Future<Output = Result<Response<Incoming>>> + Send {
        async {
            if let Err(err) = response {
                return Err(DeboaError::Request(RequestError::Send {
                    url: "".to_string(),
                    method: method.to_string(),
                    message: err.to_string(),
                }));
            }

            let response = response.unwrap();
            let status_code = response.status();
            if (!status_code.is_success()
                && !status_code.is_informational()
                && !status_code.is_redirection())
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
                return Err(DeboaError::Response(ResponseError::Receive {
                    status_code,
                    message: format!(
                        "Could not process request ({}): {}",
                        status_code,
                        String::from_utf8_lossy(&error_message)
                    ),
                }));
            }

            Ok(response)
        }
    }
}

pub(crate) mod private {
    pub trait DeboaTcpConnectionSealed {}
}
