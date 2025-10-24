use crate::ws::io::socket::{DeboaWebSocket, UpgradedIo, WebSocket};
use deboa::{response::DeboaResponse, Result};

#[deboa::async_trait]
pub trait IntoWebSocket {
    async fn into_websocket(self) -> Result<WebSocket<UpgradedIo>>;
}

#[deboa::async_trait]
impl IntoWebSocket for DeboaResponse {
    async fn into_websocket(self) -> Result<WebSocket<UpgradedIo>> {
        let upgraded = self.upgrade().await?;
        Ok(WebSocket::new(upgraded))
    }
}
