use std::sync::Arc;

use crate::http::ws::protocol::{MessageHandler, WebSocket};
use deboa::response::DeboaResponse;
use tokio::sync::Mutex;

#[deboa::async_trait]
pub trait IntoStream<H, I> 
where
    H: MessageHandler,
    I: IntoEventHandler<H> + Send + 'static,
{
    async fn into_stream(self, handler: I) -> WebSocket<H>;
}

#[deboa::async_trait]
impl<H, I> IntoStream<H, I> for DeboaResponse
where
    H: MessageHandler,
    I: IntoEventHandler<H> + Send + 'static,
{
    async fn into_stream(self, handler: I) -> WebSocket<H>
    {
        let upgraded = self.upgrade().await.expect("Failed to upgrade connection");
        WebSocket::new(upgraded, handler.into_event_handler())
    }
}

pub trait IntoEventHandler<H> where H: MessageHandler {
    fn into_event_handler(self) -> Arc<Mutex<H>>;
}

impl<H> IntoEventHandler<H> for H where H: MessageHandler {
    fn into_event_handler(self) -> Arc<Mutex<H>> {
        Arc::new(Mutex::new(self))
    }
}