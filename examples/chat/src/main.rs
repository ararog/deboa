use std::{io::stdin};

use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    protocol::{Message, MessageHandler, WebSocketRead, WebSocketWrite},
    request::WebsocketRequestBuilder,
    response::IntoStream,
};
use tokio::sync::{mpsc::{channel, Sender}};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let (tx, mut rx) = channel::<Message>(100);

    let handler = ChatHandler { tx: tx.clone() };

    let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream(handler)
        .await;

    let (mut reader, mut writer) = response.split();

    tokio::spawn(async move {
        if let Err(e) = reader.read_messages().await {
            println!("Failed to read messages: {}", e);
        }
    });

    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
          if let Err(e) = writer.write_message(message).await {
            println!("Failed to write message: {}", e);
            break;
          }
        }
    });

    loop {
        println!("You: ");
        let mut message = String::new(); // Create a mutable String to store the input
        let result = stdin().read_line(&mut message);
        if result.is_err() {
            break;
        }

        if message.trim() == "exit" {
            let result = tx
                .send(Message::Close(1000, "Normal Closure".to_string()))
                .await;
            if result.is_err() {
                return Err(deboa::errors::DeboaError::WebSocket {
                    message: "Failed to send close connection".to_string(),
                });
            }
            break;
        }

        let result = tx.send(Message::Text(message)).await;
        if result.is_err() {
            break;
        }
    }

    Ok(())
}

struct ChatHandler {
    tx: Sender<Message>,
}

#[deboa::async_trait]
impl MessageHandler for ChatHandler {
    async fn on_open(&mut self) -> Result<()> {
        println!("Connection opened");
        Ok(())
    }

    async fn on_message(&mut self, message: Option<Message>) -> Result<()> {
        match message {
            Some(Message::Text(data)) => print!("Server:\n{}\n", data),
            Some(Message::Binary(data)) => println!("Received binary message: {}", data.len()),
            Some(Message::Close(code, reason)) => {
                println!("Connection closed: {} {}", code, reason)
            }
            Some(Message::Ping(data)) => {
                let result = self.tx.send(Message::Pong(data)).await;
                if result.is_err() {
                    println!("Failed to send pong");
                }
            }
            Some(Message::Pong(data)) => println!("Received pong: {}", data.len()),
            _ => {}
        }

        Ok(())
    }

    async fn on_error(&mut self) -> Result<()> {
        println!("Connection error");
        Ok(())
    }

    async fn on_close(&mut self) -> Result<()> {
        println!("Connection closed");
        Ok(())
    }
}
