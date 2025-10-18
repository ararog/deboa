use std::{io::stdin};

use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    protocol::{Message, WebSocketRead, WebSocketWrite},
    request::WebsocketRequestBuilder,
    response::IntoStream,
};
use tokio::sync::{mpsc::channel};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let (tx, mut rx) = channel::<Message>(100);

    let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream()
        .await;

    let (mut reader, mut writer) = response.split();
    let sender = tx.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(Some(message)) = reader.read_message().await {
                match message {
                    Message::Text(data) => print!("Server:\n{}\n", data),
                    Message::Binary(data) => {
                        println!("Received binary message: {}", data.len())
                    }
                    Message::Close(code, reason) => {
                        println!("Connection closed: {} {}", code, reason)
                    }
                    Message::Ping(data) => {
                        let result = sender.send(Message::Pong(data)).await;
                        if result.is_err() {
                            println!("Failed to send pong");
                        }
                    }
                    Message::Pong(data) => println!("Received pong: {}", data.len()),
                    _ => {}
                }
            } else {
                break;
            }

            if let Some(message) = rx.recv().await {
                if let Err(e) = writer.write_message(message).await {
                    println!("Failed to write message: {}", e);
                    break;
                }
            } else {
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
