use async_trait::async_trait;
use bytes::Bytes;
use http::{Request, Response, StatusCode, Version};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;

use crate::{
    cert::ClientCert,
    client::conn::BaseHttpConnection,
    errors::{DeboaError, RequestError, ResponseError},
    Result, MAX_ERROR_MESSAGE_SIZE,
};

#[async_trait]
/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaTcpConnection: private::DeboaTcpConnectionSealed {
    type Sender;

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `is_secure` - Whether the connection is secure.
    /// * `host` - The host to connect.
    /// * `client_cert` - The client certificate to use.
    ///
    /// # Errors
    ///
    /// * `DeboaError` - If the connection fails.
    ///
    /// # Returns
    ///
    /// * `Result<BaseHttpConnection<Self::Sender>>` - The connection or error.
    ///
    async fn connect(
        is_secure: bool,
        host: &str,
        port: u16,
        client_cert: &Option<ClientCert>,
    ) -> Result<BaseHttpConnection<Self::Sender>>;

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
        method: &str,
        response: std::result::Result<Response<Incoming>, hyper::Error>,
    ) -> Result<Response<Incoming>> {
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

pub(crate) mod private {
    pub trait DeboaTcpConnectionSealed {}
}
