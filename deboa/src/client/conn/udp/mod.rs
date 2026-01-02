use async_trait::async_trait;
use bytes::{Buf, Bytes, BytesMut};
#[cfg(feature = "http3-tokio")]
use h3::{client::RequestStream, error::StreamError};
use h3_quinn::RecvStream;
#[cfg(feature = "http3-tokio")]
use http::{Request, Response, StatusCode, Version};
use http_body_util::Full;

use crate::{
    cert::Identity,
    client::conn::BaseHttpConnection,
    errors::{DeboaError, ResponseError},
    Result, MAX_ERROR_MESSAGE_SIZE,
};

#[async_trait]
/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaUdpConnection: private::DeboaUdpConnectionSealed {
    type Sender;

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `is_secure` - Whether the connection is secure.
    /// * `host` - The host to connect.
    /// * `port` - The port to connect.
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
        host: &str,
        port: u16,
        client_cert: &Option<Identity>,
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
    /// * `Result<Response<Full<Bytes>>>` - The response or error.
    ///
    async fn send_request(&mut self, request: Request<()>) -> Result<Response<Full<Bytes>>>;

    /// Process a response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response to process.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Full<Bytes>>>` - The response or error.
    ///
    async fn process_response(
        &self,
        response: std::result::Result<Response<()>, StreamError>,
        mut stream: RequestStream<RecvStream, Bytes>,
    ) -> Result<Response<Full<Bytes>>> {
        let response = response.unwrap();

        let status_code = response.status();

        if (!status_code.is_success()
            && !status_code.is_informational()
            && !status_code.is_redirection())
            || status_code == StatusCode::TOO_MANY_REQUESTS
        {
            let mut error_message = Vec::new();
            let mut downloaded = 0;
            while let Ok(Some(chunk)) = stream
                .recv_data()
                .await
            {
                if downloaded + error_message.len() > MAX_ERROR_MESSAGE_SIZE {
                    break;
                }
                error_message.extend_from_slice(chunk.chunk());
                downloaded += error_message.len();
            }

            return Err(DeboaError::Response(ResponseError::Receive {
                status_code,
                message: format!(
                    "Could not process response ({}): {}",
                    status_code,
                    String::from_utf8_lossy(&error_message)
                ),
            }));
        }

        let mut response_builder = Response::builder().status(status_code);
        let headers = response_builder
            .headers_mut()
            .unwrap();
        *headers = response
            .headers()
            .clone();
        let mut body = BytesMut::new();
        while let Ok(Some(chunk)) = stream
            .recv_data()
            .await
        {
            body.extend_from_slice(chunk.chunk());
        }

        let response = response_builder.body(Full::new(body.freeze()));
        if let Err(err) = response {
            return Err(DeboaError::Response(ResponseError::Receive {
                status_code,
                message: format!("Could not process response ({}): {}", status_code, err),
            }));
        }

        Ok(response.unwrap())
    }
}

pub(crate) mod private {
    pub trait DeboaUdpConnectionSealed {}
}
