use tokio::io::{AsyncReadExt, ReadHalf};
use ws_framer::{WsFrame, WsRxFramer};

use crate::ws::{io::socket::UpgradedIo, protocol::Message};

use deboa::{errors::{DeboaError, WebSocketError}, Result};

#[deboa::async_trait]
pub trait WebSocketRead {
    async fn read_message(&mut self) -> Result<Option<Message>>;
}

pub struct WebSocketReader<T> {
    stream: T,
}

impl WebSocketReader<ReadHalf<UpgradedIo>> {
    pub fn new(stream: ReadHalf<UpgradedIo>) -> Self {
        Self { stream }
    }
}

#[deboa::async_trait]
impl WebSocketRead for WebSocketReader<ReadHalf<UpgradedIo>> {
    async fn read_message(&mut self) -> Result<Option<Message>> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);

        let bytes_read = self.stream.read(rx_framer.mut_buf()).await;
        if bytes_read.is_err() {
            return Err(DeboaError::WebSocket(WebSocketError::ReceiveMessage {
                message: "Failed to read message".to_string(),
            }));
        }

        let bytes_read = bytes_read.unwrap();
        rx_framer.revolve_write_offset(bytes_read);
        let res = rx_framer.process_data();
        let message = if let Some(frame) = res {
            #[allow(clippy::collapsible_match)]
            match frame {
                WsFrame::Text(data) => Some(Message::Text(data.to_string())),
                WsFrame::Binary(data) => Some(Message::Binary(data.to_vec())),
                WsFrame::Close(code, reason) => Some(Message::Close(code, reason.to_string())),
                WsFrame::Ping(data) => Some(Message::Ping(data.to_vec())),
                _ => None,
            }
        } else {
            None
        };

        Ok(message)
    }
}


