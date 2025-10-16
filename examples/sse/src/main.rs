use deboa::{Deboa, Result};
use deboa_extras::http::sse::response::{EventHandler, IntoStream};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let response = client.execute("https://sse.dev/test").await?.into_stream();

    let handler = SSEHandler;

    response
        .poll_event(handler)
        .await?;

    println!("Connection closed");

    Ok(())
}

pub struct SSEHandler;

#[deboa::async_trait]
impl EventHandler for SSEHandler {
    async fn on_event(&mut self, event: &str) -> Result<()> {
        println!("event: {}", event);
        Ok(())
    }
}
