use deboa::{response::DeboaResponse, Result};
use http_body_util::BodyExt;
use mime_typed::MimeStrExt;

pub struct SSE {
    response: DeboaResponse,
}

pub trait IntoStream {
    fn into_stream(self) -> SSE;
}

impl IntoStream for DeboaResponse {
    fn into_stream(self) -> SSE {
        SSE { response: self }
    }
}

#[deboa::async_trait]
pub trait EventHandler {
    async fn on_event(&mut self, event: &str) -> Result<()>;
}

impl SSE {
    pub async fn poll_event<E>(self, mut handler: E) -> Result<()>
    where
        E: EventHandler + Send + Sync + 'static,
    {
        let header = self.response.headers().get(http::header::CONTENT_TYPE);
        if let Some(header) = header {
            if header == mime_typed::TextEventStream::MIME_STR {
                let mut stream = self.response.stream();
                while let Some(frame) = stream.frame().await {
                    let frame = frame.unwrap();
                    if let Some(event) = frame.data_ref() {
                        handler
                            .on_event(String::from_utf8_lossy(event).as_ref())
                            .await?;
                    }
                }
            } else {
                return Err(deboa::errors::DeboaError::SSE {
                    message: "Content type is not text/event-stream".to_string(),
                });
            }
        } else {
            return Err(deboa::errors::DeboaError::SSE {
                message: "Missing content type".to_string(),
            });
        }

        Ok(())
    }
}
