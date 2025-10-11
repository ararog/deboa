use deboa::{async_trait, response::DeboaResponse, Result};
use http_body_util::BodyExt;

#[async_trait]
pub trait EventListener : Send + Sync + 'static {
    async fn poll_event<F>(&mut self, _on_event: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()> + Send + Sync + 'static,
    {
        unimplemented!()
    }
}

#[async_trait]
impl EventListener for DeboaResponse {
    async fn poll_event<F>(&mut self, mut on_event: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()> + Send + Sync + 'static,
    {
        let header = self.headers().get(http::header::CONTENT_TYPE);
        if let Some(header) = header {
            if header == "text/event-stream" {
                while let Some(frame) = self.stream().frame().await {
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
