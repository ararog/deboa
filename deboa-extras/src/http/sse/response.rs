use deboa::{response::DeboaResponse, Result};
use futures::StreamExt;
pub use http_body_util::BodyExt;
use mime_typed::MimeStrExt;

pub struct SSE {
    response: DeboaResponse,
}

/// Trait to convert a DeboaResponse into a SSE stream.
pub trait IntoStream {
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
    /// let sse = response.into_stream();
    /// ```
    fn into_stream(self) -> SSE;
}

impl IntoStream for DeboaResponse {
    fn into_stream(self) -> SSE {
        SSE { response: self }
    }
}

/// Trait to handle SSE events.
#[deboa::async_trait]
pub trait EventHandler {
    /// Handles an SSE event.
    ///
    /// # Arguments
    ///
    /// * `event` - A string slice that holds the event data.
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// let handler = MyEventHandler;
    /// let result = handler.on_event("event data");
    /// ```
    async fn on_event(&mut self, event: &str) -> Result<()>;
}

impl SSE {
    pub async fn poll_event<E>(self, mut handler: E) -> Result<()>
    where
        E: EventHandler + Send + Sync + 'static,
    {
        let header = self.response.headers().get(http::header::CONTENT_TYPE);
        if header.is_none() {
            return Err(deboa::errors::DeboaError::SSE {
                message: "Missing content type".to_string(),
            });
        }

        let header = header.unwrap();

        if header != mime_typed::TextEventStream::MIME_STR {
            return Err(deboa::errors::DeboaError::SSE {
                message: "Content type is not text/event-stream".to_string(),
            });
        }

        let mut stream = self.response.stream();
        while let Some(frame) = stream.next().await {
            if let Ok(event) = frame {
                handler
                    .on_event(&String::from_utf8_lossy(event.as_ref()))
                    .await?;
            }
        }

        Ok(())
    }
}
