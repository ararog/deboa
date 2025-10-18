use crate::http::ws::protocol::WebSocket;
use deboa::response::DeboaResponse;

#[deboa::async_trait]
pub trait IntoStream {
    async fn into_stream(self) -> WebSocket;
}

#[deboa::async_trait]
impl IntoStream for DeboaResponse {
    async fn into_stream(self) -> WebSocket {
        let upgraded = self.upgrade().await.expect("Failed to upgrade connection");
        WebSocket::new(upgraded)
    }
}
