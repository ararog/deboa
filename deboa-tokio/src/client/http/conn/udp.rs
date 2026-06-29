use crate::{
    client::conn::{BaseHttpConnection, ConnectionConfig},
    Result, MAX_ERROR_MESSAGE_SIZE,
};
use bytes::{Buf, Bytes};
use deboa::errors::{DeboaError, ResponseError};
use h3::client::RequestStream;
use h3_quinn::RecvStream;
use http::response::Parts;
use http::{Request, Response, StatusCode, Version};
use http_body::Body;
use hyper_body_utils::HttpBody;
use std::future::Future;

/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
///
pub trait DeboaUdpConnection: private::DeboaUdpConnectionSealed {
    /// The sender type.
    type Sender;
    /// The request body type.
    type ReqBody: Body + Unpin;
    /// The response body type.
    type ResBody: Body + Unpin;

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
    /// * `Result<BaseHttpConnection<Self::Sender, Self::ReqBody, Self::ResBody>>` - The connection or error.
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
    /// * `response` - The response to process.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Self::ResBody>>` - The response or error.
    ///
    fn process_response(
        &self,
        parts: Parts,
        mut stream: RequestStream<RecvStream, Bytes>,
    ) -> impl Future<Output = Result<Response<HttpBody>>> + Send {
        async move {
            let status_code = parts.status;

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
                        "Could not process request ({}): {}",
                        status_code,
                        String::from_utf8_lossy(&error_message)
                    ),
                }));
            }

            let body = HttpBody::from_quic_client(stream);
            let response = Response::from_parts(parts, body);

            Ok(response)
        }
    }
}

pub(crate) mod private {
    pub trait DeboaUdpConnectionSealed {}
}
