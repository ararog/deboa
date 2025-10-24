use tokio::io::{AsyncWriteExt, WriteHalf};
use ws_framer::{WsFrame, WsTxFramer};

use crate::ws::{io::socket::UpgradedIo, protocol::Message};

use deboa::Result;

#[deboa::async_trait]
pub trait WebSocketWrite {
    type Stream;

    async fn write_message(&mut self, message: Message) -> Result<()>;

    async fn send_close(&mut self, code: u16, reason: &str) -> Result<()>;

    async fn send_text(&mut self, message: &str) -> Result<()>;

    async fn send_binary(&mut self, message: &[u8]) -> Result<()>;

    async fn send_ping(&mut self, message: &[u8]) -> Result<()>;

    async fn send_pong(&mut self, message: &[u8]) -> Result<()>;
}

pub struct WebSocketWriter<T> {
    stream: T,
}

impl WebSocketWriter<WriteHalf<UpgradedIo>> {
    pub fn new(stream: WriteHalf<UpgradedIo>) -> Self {
        Self { stream }
    }
}

#[deboa::async_trait]
impl WebSocketWrite for WebSocketWriter<WriteHalf<UpgradedIo>> {
    type Stream = WriteHalf<UpgradedIo>;

    async fn write_message(&mut self, message: Message) -> Result<()> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let result = match message {
            Message::Text(data) => {
                self.stream
                    .write_all(tx_framer.frame(WsFrame::Text(&data)))
                    .await
            }
            Message::Binary(data) => {
                self.stream
                    .write_all(tx_framer.frame(WsFrame::Binary(&data)))
                    .await
            }
            Message::Close(code, reason) => {
                self.stream
                    .write_all(tx_framer.frame(WsFrame::Close(code, &reason)))
                    .await
            }
            Message::Ping(data) => {
                self.stream
                    .write_all(tx_framer.frame(WsFrame::Ping(&data)))
                    .await
            }
            _ => Ok(()),
        };

        if result.is_err() {
            return Err(deboa::errors::DeboaError::WebSocket {
                message: "Failed to send frame".to_string(),
            });
        }

        Ok(())
    }

    async fn send_close(&mut self, code: u16, reason: &str) -> Result<()> {
        self.write_message(Message::Close(code, reason.to_string()))
            .await
    }

    async fn send_text(&mut self, message: &str) -> Result<()> {
        self.write_message(Message::Text(message.to_string())).await
    }

    async fn send_binary(&mut self, message: &[u8]) -> Result<()> {
        self.write_message(Message::Binary(message.to_vec())).await
    }

    async fn send_ping(&mut self, message: &[u8]) -> Result<()> {
        self.write_message(Message::Ping(message.to_vec())).await
    }

    async fn send_pong(&mut self, message: &[u8]) -> Result<()> {
        self.write_message(Message::Pong(message.to_vec())).await
    }
}
