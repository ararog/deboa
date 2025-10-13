use deboa::{errors::DeboaError, response::DeboaResponse, Result};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct WebSocket {
    stream: TokioIo<Upgraded>,
}

#[deboa::async_trait]
pub trait IntoStream {
    async fn into_stream(self) -> WebSocket;
}

#[deboa::async_trait]
impl IntoStream for DeboaResponse {
    async fn into_stream(self) -> WebSocket {
        WebSocket {
            stream: self.upgrade().await.expect("Failed to upgrade connection"),
        }
    }
}

impl WebSocket {
    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        if let Err(e) = self.stream.write(message.as_bytes()).await {
            return Err(DeboaError::WebSocket {
                message: e.to_string(),
            });
        }
        Ok(())
    }

    pub async fn poll_message<F>(&mut self, mut on_event: F) -> Result<()>
    where
        F: FnMut(&str) -> Result<()> + Send + Sync + 'static,
    {
        let mut vec = vec![0; 1024];
        loop {
            let result = self.stream.read(&mut vec).await;

            if result.is_err() {
                eprintln!("Failed to read from stream: {}", result.unwrap_err());
                break;
            }

            let size = result.unwrap();
            if size == 0 {
                break;
            }

            on_event(String::from_utf8_lossy(&vec[..size]).as_ref())?;
        }

        Ok(())
    }
}
