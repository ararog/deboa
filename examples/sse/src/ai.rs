use deboa::{Deboa, request::DeboaRequest, response::DeboaResponse};
use deboa_extras::http::serde::json::JsonBody;
use futures::channel::mpsc;
use http::header;
use serde::{Deserialize, Serialize};
use sipper::{Never, Sipper, StreamExt, sipper};
use std::fmt;

const API_KEY: &str = "YOUR_OPENAI_API_KEY";

pub fn ai() -> impl Sipper<Never, Event> {
    sipper(async |mut output| {
        let (sender, mut receiver) = mpsc::channel::<Message>(100);
        let mut client = Deboa::new();
        output.send(Event::Connected(Connection(sender))).await;
        loop {
            if let Some(Message::User(message)) = receiver.next().await {
                let response = make_request(&mut client, message.as_str()).await;
                let mut stream = response.unwrap().stream();
                while let Some(message) = stream.next().await {
                    if let Ok(frame) = message {
                        let text_message = String::from_utf8_lossy(frame.as_ref()).to_string();
                        let message = Message::new(&text_message).expect("Invalid message");
                        output
                            .send(Event::MessageReceived(message))
                            .await
                    }
                }
            }
        }
    })
}

async fn make_request(client: &mut Deboa, message: &str) -> Result<DeboaResponse, String> {
    let response = DeboaRequest::post("https://api.openai.com/v1/chat/completions")
        .unwrap()
        .bearer_auth(API_KEY)
        .header(header::CONTENT_TYPE, "application/json")
        .body_as(
            JsonBody,
            &Prompt {
                model: "gpt-3.5-turbo".to_string(),
                messages: vec![PromptMessage {
                    role: "user".to_string(),
                    content: message.to_string(),
                }],
            },
        )
        .unwrap()
        .go(client)
        .await;

    if let Err(message) = response {
        println!("Failed to connect to echo server: {}", message);
        Err(message.to_string())
    } else {
        Ok(response.unwrap())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Prompt {
    pub model: String,
    pub messages: Vec<PromptMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum Event {
    Connected(Connection),
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

    pub fn as_str(&self) -> &str {
        match self {
            Message::User(message) => message.as_str(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
