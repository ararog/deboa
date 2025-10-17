use std::sync::Arc;

use deboa::Result;
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    sync::Mutex,
};
use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

type Io = TokioIo<Upgraded>;

/// WebSocket struct
pub struct WebSocket<H>
where
    H: MessageHandler,
{
    stream: Io,
    handler: Arc<Mutex<H>>,
}

impl<H> WebSocket<H>
where
    H: MessageHandler,
{
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
    pub fn new(stream: Io, handler: Arc<Mutex<H>>) -> Self {
        Self { stream, handler }
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
        WebSocketReader<ReadHalf<Io>, H>,
        WebSocketWriter<WriteHalf<Io>, H>,
    ) {
        let (reader, writer) = tokio::io::split(self.stream);
        let handler_reader = Arc::clone(&self.handler);
        let handler_writer = Arc::clone(&self.handler);
        (
            WebSocketReader {
                stream: reader,
                handler: handler_reader,
            },
            WebSocketWriter {
                stream: writer,
                handler: handler_writer,
            },
        )
    }
}

#[deboa::async_trait]
pub trait WebSocketRead<T, H>
where
    T: AsyncReadExt + Unpin + Send,
    H: MessageHandler,
{
    fn handler(&mut self) -> Arc<Mutex<H>>;

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

    async fn read_messages(&mut self) -> Result<()> {
        loop {
            let message = self.read_message().await?;
            if message.is_none() {
                break;
            }

            self.handler().lock().await.on_message(message).await?;
        }
        Ok(())
    }
}

#[deboa::async_trait]
pub trait WebSocketWrite<T, H>
where
    T: AsyncWriteExt + Unpin + Send,
    H: MessageHandler,
{
    fn handler(&mut self) -> Arc<Mutex<H>>;

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

pub struct WebSocketReader<T, H>
where
    T: AsyncReadExt + Unpin + Send,
    H: MessageHandler,
{
    stream: T,
    handler: Arc<Mutex<H>>,
}

impl<T, H> WebSocketReader<T, H>
where
    T: AsyncReadExt + Unpin + Send,
    H: MessageHandler,
{
    pub fn new(stream: T, handler: Arc<Mutex<H>>) -> Self {
        Self { stream, handler }
    }
}

#[deboa::async_trait]
impl<T, H> WebSocketRead<T, H> for WebSocketReader<T, H>
where
    T: AsyncReadExt + Unpin + Send,
    H: MessageHandler,
{
    fn handler(&mut self) -> Arc<Mutex<H>> {
        self.handler.clone()
    }

    fn stream(&mut self) -> &mut T {
        &mut self.stream
    }
}

pub struct WebSocketWriter<T, H>
where
    T: AsyncWriteExt + Unpin + Send,
    H: MessageHandler,
{
    stream: T,
    handler: Arc<Mutex<H>>,
}

impl<T, H> WebSocketWriter<T, H>
where
    T: AsyncWriteExt + Unpin + Send,
    H: MessageHandler,
{
    pub fn new(stream: T, handler: Arc<Mutex<H>>) -> Self {
        Self { stream, handler }
    }
}

#[deboa::async_trait]
impl<T, H> WebSocketWrite<T, H> for WebSocketWriter<T, H>
where
    T: AsyncWriteExt + Unpin + Send,
    H: MessageHandler,
{
    fn handler(&mut self) -> Arc<Mutex<H>> {
        self.handler.clone()
    }

    fn stream(&mut self) -> &mut T {
        &mut self.stream
    }
}

/// MessageHandler trait
#[deboa::async_trait]
pub trait MessageHandler: Send + Sync + 'static {
    /// on_open method
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure.
    /// 
    async fn on_open(&mut self) -> Result<()>;
    /// on_message method
    /// 
    /// # Arguments
    /// 
    /// * `message` - A string slice that holds the message data.
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure.
    /// 
    async fn on_message(&mut self, message: Option<Message>) -> Result<()>;
    /// on_error method
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure.
    /// 
    async fn on_error(&mut self) -> Result<()>;
    /// on_close method
    /// 
    /// # Returns
    /// 
    /// A Result indicating success or failure.
    /// 
    async fn on_close(&mut self) -> Result<()>;
}

pub struct EmptyHandler;

#[deboa::async_trait]
impl MessageHandler for EmptyHandler {
    async fn on_open(&mut self) -> Result<()> {
        Ok(())
    }

    async fn on_message(&mut self, _message: Option<Message>) -> Result<()> {
        Ok(())
    }

    async fn on_error(&mut self) -> Result<()> {
        Ok(())
    }

    async fn on_close(&mut self) -> Result<()> {
        Ok(())
    }
}
