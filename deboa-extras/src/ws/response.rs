use crate::ws::io::socket::{DeboaWebSocket, UpgradedIo, WebSocket};
use deboa::{response::DeboaResponse, Result};

/// Trait for converting a DeboaResponse into a WebSocket
#[deboa::async_trait]
pub trait IntoWebSocket {
    /// Converts a DeboaResponse into a WebSocket
    ///
    /// # Arguments
    ///
    /// * `self` - The DeboaResponse to convert
    ///
    /// # Returns
    ///
    /// A Result containing the WebSocket
    ///
    /// # Example
    ///
    /// ``` compile_fail
    /// use deboa::{Deboa, Result, request::{IntoUrl, DeboaRequestBuilder}};
    /// use deboa_extras::http::ws::request::{WebsocketRequestBuilder};
    ///
    /// let mut client = Deboa::new();
    /// let builder = DeboaRequestBuilder::websocket("ws://example.com").unwrap();
    /// let response = builder.go(&mut client).await.unwrap();
    /// let websocket = response.into_websocket().unwrap();
    ///
    /// loop {
    ///     if let Ok(Some(message)) = websocket.read_message().await {
    ///         println!("message: {}", message);
    ///     }
    /// }
    /// ```
    async fn into_websocket(self) -> Result<WebSocket<UpgradedIo>>;
}

#[deboa::async_trait]
impl IntoWebSocket for DeboaResponse {
    async fn into_websocket(self) -> Result<WebSocket<UpgradedIo>> {
        let upgraded = self.upgrade().await?;
        Ok(WebSocket::new(upgraded))
    }
}
