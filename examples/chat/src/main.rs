use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{request::WebsocketRequestBuilder, response::IntoStream};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let mut response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream()
        .await;

    response.send_message("Hello, world!").await?;

    response
        .poll_message(|message| {
            println!("Received message: {}", message);
            Ok(())
        })
        .await?;

    Ok(())
}
