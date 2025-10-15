use std::{io::stdin, thread::spawn};

use deboa::{request::DeboaRequestBuilder, Deboa, Result};
use deboa_extras::http::ws::{request::WebsocketRequestBuilder, response::{IntoStream, MessageHandler}};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let mut response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
        .go(&mut client)
        .await?
        .into_stream()
        .await;

    let mut handler = ChatHandler;
      
    println!("Please enter some text:");
    let mut message = String::new(); // Create a mutable String to store the input
    stdin().read_line(&mut message); 
    response.send_message(&message).await;
    
    while let Ok(Some(message)) = response.read_message().await {
        handler.on_message(message);
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
