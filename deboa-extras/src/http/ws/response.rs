use crate::http::ws::protocol::{DeboaWebSocket, UpgradedIo, WebSocket};
use deboa::{Result, response::DeboaResponse};

#[deboa::async_trait]
pub trait IntoStream {
    async fn into_stream(self) -> Result<WebSocket<UpgradedIo>>;
}

#[deboa::async_trait]
impl IntoStream for DeboaResponse {
    async fn into_stream(self) -> Result<WebSocket<UpgradedIo>> {
        let upgraded = self.upgrade().await?;
        Ok(WebSocket::new(upgraded))
    }
}
