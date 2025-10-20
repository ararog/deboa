use deboa::{Deboa, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    protocol::{self, WebSocketRead, WebSocketWrite},
    request::WebsocketRequestBuilder,
    response::IntoStream,
};
use iced::futures;
use iced::widget::text;

use futures::channel::mpsc;
use sipper::{Never, Sipper, StreamExt, sipper};

use std::fmt;

pub fn connect() -> impl Sipper<Never, Event> {
    sipper(async |mut output| {
        loop {
            let mut client = Deboa::new();
            let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")
                .unwrap()
                .go(&mut client)
                .await;

            if let Err(message) = response {
                println!("Failed to connect to echo server: {}", message);

                continue;
            }

            let (mut websocket, mut input) = match response.unwrap().into_stream().await {
                Ok(websocket) => {
                    let (sender, receiver) = mpsc::channel(100);

                    output.send(Event::Connected(Connection(sender))).await;

                    (websocket, receiver)
                }
                Err(message) => {
                    println!("Failed to connect to echo server: {}", message);

                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

                    continue;
                }
            };

            loop {
                let result = websocket.read_message().await;
                if let Err(message) = result {
                    println!("Failed to read message from echo server: {}", message);

                    output.send(Event::Disconnected).await;
                    break;
                }

                match result.unwrap() {
                    Some(message) => {
                        if let protocol::Message::Text(message) = message {
                            output
                                .send(Event::MessageReceived(Message::User(
                                    format!("Server: {}", message).to_string(),
                                )))
                                .await;
                        }
                    }
                    None => {
                        output.send(Event::Disconnected).await;
                        break;
                    }
                }

                if let Some(message) = input.next().await {
                    let result = websocket
                        .write_message(protocol::Message::Text(message.to_string()))
                        .await;
                    if result.is_err() {
                        output.send(Event::Disconnected).await;
                        break;
                    }
                }
            }
        }
    })
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
    Disconnected,
    MessageReceived(Message),
}

#[derive(Debug, Clone)]
pub struct Connection(mpsc::Sender<Message>);

impl Connection {
    pub fn send(&mut self, message: Message) {
        self.0
            .try_send(message)
            .expect("Send message to echo server");
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected,
    Disconnected,
    User(String),
}

impl Message {
    pub fn new(message: &str) -> Option<Self> {
        if message.is_empty() {
            None
        } else {
            Some(Self::User(message.to_string()))
        }
    }

    pub fn connected() -> Self {
        Message::Connected
    }

    pub fn disconnected() -> Self {
        Message::Disconnected
    }

    pub fn as_str(&self) -> &str {
        match self {
            Message::Connected => "Connected successfully!",
            Message::Disconnected => "Connection lost... Retrying...",
            Message::User(message) => message.as_str(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<'a> text::IntoFragment<'a> for &'a Message {
    fn into_fragment(self) -> text::Fragment<'a> {
        text::Fragment::Borrowed(self.as_str())
    }
}
