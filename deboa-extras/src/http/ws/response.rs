use crate::http::ws::protocol::{MessageHandler, WebSocket};
use deboa::response::DeboaResponse;

#[deboa::async_trait]
pub trait IntoStream {
    async fn into_stream<H>(self, handler: H) -> WebSocket<H>
    where
        H: MessageHandler;
}

#[deboa::async_trait]
impl IntoStream for DeboaResponse {
    async fn into_stream<H>(self, handler: H) -> WebSocket<H>
    where
        H: MessageHandler,
    {
        let upgraded = self.upgrade().await.expect("Failed to upgrade connection");
        WebSocket::new(upgraded, handler)
    }
}
