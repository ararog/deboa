use deboa::{errors::DeboaError, response::DeboaResponse, Result};
pub use http_body_util::BodyExt;
use mime_typed::MimeStrExt;

use crate::http::sse::{
    io::stream::ServerEventStream,
};

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
    /// let response = DeboaResponse::new();
    /// let sse = response.into_event_stream();
    /// ```
    fn into_event_stream(self) -> Result<ServerEventStream>;
}

impl IntoEventStream for DeboaResponse {
    fn into_event_stream(self) -> Result<ServerEventStream> {
        let header = self.headers().get(http::header::CONTENT_TYPE);
        if header.is_none() {
            return Err(DeboaError::SSE {
                message: "Missing content type".to_string(),
            });
        }

        let header = header.unwrap();

        if header != mime_typed::TextEventStream::MIME_STR {
            return Err(DeboaError::SSE {
                message: "Content type is not text/event-stream".to_string(),
            });
        }

        Ok(ServerEventStream::new(self.inner_body()))
    }
}
