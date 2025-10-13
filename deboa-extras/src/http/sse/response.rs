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

impl SSE {
    pub async fn poll_event<F>(self, mut on_event: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()> + Send + Sync + 'static,
    {
        let header = self.response.headers().get(http::header::CONTENT_TYPE);
        if let Some(header) = header {
            if header == mime_typed::TextEventStream::MIME_STR {
                let mut stream = self.response.stream();
                while let Some(frame) = stream.frame().await {
                    let frame = frame.unwrap();
                    if let Some(event) = frame.data_ref() {
                        on_event(String::from_utf8_lossy(event).as_ref())?;
                    }
                }
            }
        }

        Ok(())
    }
}
