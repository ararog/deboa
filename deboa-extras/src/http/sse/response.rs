use deboa::{errors::DeboaError, response::DeboaResponse, Result};
pub use http_body_util::BodyExt;
use mime_typed::MimeStrExt;

use crate::http::sse::io::stream::ServerEventStream;

/// Trait to convert a DeboaResponse into a SSE stream.
pub trait IntoEventStream {
    /// Converts a DeboaResponse into a SSE stream.
    ///
    /// # Returns
    ///
    /// A SSE struct.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Deboa, Result};
    /// use deboa_extras::http::sse::response::{IntoEventStream};
    ///
    /// let mut client = Deboa::new();
    ///
    /// let response = client.execute("https://sse.dev/test").await?.into_event_stream();
    ///
    /// while let Some(event) = response.next().await {
    ///     println!("event: {}", event);
    /// }
    /// ```
    fn into_event_stream(self) -> Result<ServerEventStream>;
}

impl IntoEventStream for DeboaResponse {
    fn into_event_stream(self) -> Result<ServerEventStream> {
        let header = self.headers().get(http::header::CONTENT_TYPE);
        if header.is_none() {
            return Err(DeboaError::Header {
                message: "Missing content type".to_string(),
            });
        }

        let header = header.unwrap();

        let message = "Content type is not text/event-stream".to_string();

        let header = header.to_str();

        if let Err(_error) = header {
            return Err(DeboaError::Header {
                message,
            });
        }

        if !header.unwrap().contains(mime_typed::TextEventStream::MIME_STR) {
            return Err(DeboaError::Header {
                message,
            });
        }

        Ok(ServerEventStream::new(self.inner_body()))
    }
}
