use std::{io::stdin, sync::{Arc, RwLock}};

use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    request::WebsocketRequestBuilder,
    response::{IntoStream, Message, MessageHandler},
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream()
        .await;

    let (mut reader, mut writer) = response.split();

    let handler = Arc::new(RwLock::new(ChatHandler));

    let reader_handler = Arc::clone(&handler);

    let reader_task = tokio::spawn(async move {
        while let Ok(Some(message)) = reader.read_message().await {
            reader_handler.write().unwrap().on_message(message);
        }
    });

    loop {
        println!("Please enter some text:");
        let mut message = String::new(); // Create a mutable String to store the input
        let result =  stdin().read_line(&mut message);
        if result.is_err() {
            break;
        }

        if message.trim() == "exit" {
            let result = writer.send_close(1000, "Normal Closure").await;
            if result.is_err() {
                return Err(deboa::errors::DeboaError::WebSocket {
                    message: "Failed to send close connection".to_string(),
                });
            }
            break;
        }

        let result = writer.send_text(&message).await;
        if result.is_err() {
            return Err(deboa::errors::DeboaError::WebSocket {
                message: "Failed to send text message".to_string(),
            });
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

#[derive(Clone)]
struct ChatHandler;

#[deboa::async_trait]
impl MessageHandler for ChatHandler {
    fn on_open(&mut self) {
        println!("Connection opened");
    }

    fn on_message(&mut self, message: Message) {
        match message {
            Message::Text(data) => println!("Received message: {}", data),
            Message::Binary(data) => println!("Received binary message: {}", data.len()),
            Message::Close(code, reason) => println!("Connection closed: {} {}", code, reason),
            Message::Ping(data) => println!("Received ping: {}", data.len()),
            Message::Pong(data) => println!("Received pong: {}", data.len()),
        }
    }

    fn on_close(&mut self) {
        println!("Connection closed");
    }
}
