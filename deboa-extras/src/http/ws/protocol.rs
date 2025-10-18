use deboa::Result;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

type Io = TokioIo<Upgraded>;

/// WebSocket struct
pub struct WebSocket {
    stream: Io,
}

impl WebSocket {
    /// new method
    ///
    /// # Arguments
    ///
    /// * `stream` - A string slice that holds the stream data.
    ///
    /// # Returns
    ///
    /// A WebSocket struct.
    ///
    pub fn new(stream: Io) -> Self {
        Self { stream }
    }

    /// split method
    ///
    /// # Returns
    ///
    /// A tuple of WebSocketReader and WebSocketWriter.
    ///
    pub fn split(
        self,
    ) -> (
        WebSocketReader<ReadHalf<Io>>,
        WebSocketWriter<WriteHalf<Io>>,
    ) {
        let (reader, writer) = tokio::io::split(self.stream);
        (
            WebSocketReader { stream: reader },
            WebSocketWriter { stream: writer },
        )
    }
}

#[deboa::async_trait]
pub trait WebSocketRead<T>
where
    T: AsyncReadExt + Unpin + Send,
{
    fn stream(&mut self) -> &mut T;

    async fn read_message(&mut self) -> Result<Option<Message>> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);
        let stream = self.stream();

        let bytes_read = stream.read(rx_framer.mut_buf()).await;
        if bytes_read.is_err() {
            return Err(deboa::errors::DeboaError::WebSocket {
                message: "Failed to read message".to_string(),
            });
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

#[deboa::async_trait]
pub trait WebSocketWrite<T>
where
    T: AsyncWriteExt + Unpin + Send,
{
    fn stream(&mut self) -> &mut T;

    async fn write_message(&mut self, message: Message) -> Result<()> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let result = match message {
            Message::Text(data) => {
                self.stream()
                    .write_all(tx_framer.frame(WsFrame::Text(&data)))
                    .await
            }
            Message::Binary(data) => {
                self.stream()
                    .write_all(tx_framer.frame(WsFrame::Binary(&data)))
                    .await
            }
            Message::Close(code, reason) => {
                self.stream()
                    .write_all(tx_framer.frame(WsFrame::Close(code, &reason)))
                    .await
            }
            Message::Ping(data) => {
                self.stream()
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

/// Message enum
///
/// # Variants
///
/// * `Text(String)` - A text message.
/// * `Binary(Vec<u8>)` - A binary message.
/// * `Close(u16, String)` - A close message.
/// * `Ping(Vec<u8>)` - A ping message.
/// * `Pong(Vec<u8>)` - A pong message.
#[derive(Clone)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Close(u16, String),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
}

pub struct WebSocketReader<T>
where
    T: AsyncReadExt + Unpin + Send,
{
    stream: T,
}

impl<T> WebSocketReader<T>
where
    T: AsyncReadExt + Unpin + Send,
{
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

#[deboa::async_trait]
impl<T> WebSocketRead<T> for WebSocketReader<T>
where
    T: AsyncReadExt + Unpin + Send,
{
    fn stream(&mut self) -> &mut T {
        &mut self.stream
    }
}

pub struct WebSocketWriter<T>
where
    T: AsyncWriteExt + Unpin + Send,
{
    stream: T,
}

impl<T> WebSocketWriter<T>
where
    T: AsyncWriteExt + Unpin + Send,
{
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

#[deboa::async_trait]
impl<T> WebSocketWrite<T> for WebSocketWriter<T>
where
    T: AsyncWriteExt + Unpin + Send,
{
    fn stream(&mut self) -> &mut T {
        &mut self.stream
    }
}

