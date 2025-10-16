use std::io::stdin;

use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    request::WebsocketRequestBuilder,
    response::{IntoStream, MessageHandler},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream()
        .await;

    let mut handler = ChatHandler;

    let (mut reader, mut writer) = response.split();

    let reader_task = tokio::spawn(async move {
        while let Ok(Some(message)) = reader.read_message().await {
            handler.on_message(message);
        }
    });

    loop {
        println!("Please enter some text:");
        let mut message = String::new(); // Create a mutable String to store the input
        let result =  stdin().read_line(&mut message);
        if result.is_err() {
            break;
        }

        let result = writer.send_message(&message).await;
        if result.is_err() {
            break;
        }
    }

    let result = reader_task.await;
    if result.is_err() {
        return Err(deboa::errors::DeboaError::WebSocket {
            message: "Failed to read message".to_string(),
        });
    }

    Ok(())
}

struct ChatHandler;

impl MessageHandler for ChatHandler {
    fn on_open(&mut self) {
        println!("Connection opened");
    }

    fn on_message(&mut self, message: String) {
        println!("Received message: {}", message);
    }

    fn on_close(&mut self) {
        println!("Connection closed");
    }
}
