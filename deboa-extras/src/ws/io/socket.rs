use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project_lite::pin_project;

use hyper::upgrade::Upgraded;
#[cfg(feature = "tokio")]
use hyper_util::rt::TokioIo;
#[cfg(feature = "smol")]
use smol_hyper::rt::FuturesIo;

#[cfg(feature = "smol")]
use smol::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};

use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

use crate::{
    errors::{DeboaExtrasError, WebSocketError},
    ws::protocol::Message,
};

#[cfg(feature = "tokio")]
pub type UpgradedIo = TokioIo<Upgraded>;

#[cfg(feature = "smol")]
pub type UpgradedIo = FuturesIo<Upgraded>;

#[deboa::async_trait]
pub trait DeboaWebSocket {
    type Stream;

    fn new(stream: Self::Stream) -> Self;
    async fn read_message(&mut self) -> Result<Option<Message>, DeboaExtrasError>;
    async fn write_message(&mut self, message: Message) -> Result<(), DeboaExtrasError>;
    async fn send_close(&mut self, code: u16, reason: &str) -> Result<(), DeboaExtrasError>;
    async fn send_text(&mut self, message: &str) -> Result<(), DeboaExtrasError>;
    async fn send_binary(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError>;
    async fn send_ping(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError>;
    async fn send_pong(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError>;
}

pin_project! {
    /// WebSocket struct
    pub struct WebSocket<T>
    {
        #[pin]
        stream: T,
    }
}

#[deboa::async_trait]
impl DeboaWebSocket for WebSocket<UpgradedIo> {
    type Stream = UpgradedIo;

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
    fn new(stream: Self::Stream) -> Self {
        Self { stream }
    }

    async fn read_message(&mut self) -> Result<Option<Message>, DeboaExtrasError> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);

        let bytes_read = self.stream.read(rx_framer.mut_buf()).await;
        if bytes_read.is_err() {
            return Err(DeboaExtrasError::WebSocket(
                WebSocketError::ReceiveMessage {
                    message: "Failed to read message".to_string(),
                },
            ));
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

    async fn write_message(&mut self, message: Message) -> Result<(), DeboaExtrasError> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let result = match message {
            Message::Text(data) => self.write_all(tx_framer.frame(WsFrame::Text(&data))).await,
            Message::Binary(data) => {
                self.write_all(tx_framer.frame(WsFrame::Binary(&data)))
                    .await
            }
            Message::Close(code, reason) => {
                self.write_all(tx_framer.frame(WsFrame::Close(code, &reason)))
                    .await
            }
            Message::Ping(data) => self.write_all(tx_framer.frame(WsFrame::Ping(&data))).await,
            _ => Ok(()),
        };

        if result.is_err() {
            return Err(DeboaExtrasError::WebSocket(WebSocketError::SendMessage {
                message: "Failed to send frame".to_string(),
            }));
        }

        Ok(())
    }

    async fn send_close(&mut self, code: u16, reason: &str) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Close(code, reason.to_string()))
            .await
    }

    async fn send_text(&mut self, message: &str) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Text(message.to_string())).await
    }

    async fn send_binary(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Binary(message.to_vec())).await
    }

    async fn send_ping(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Ping(message.to_vec())).await
    }

    async fn send_pong(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Pong(message.to_vec())).await
    }
}

impl AsyncRead for WebSocket<UpgradedIo> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.project().stream.poll_read(cx, buf)
    }
}

impl AsyncWrite for WebSocket<UpgradedIo> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::result::Result<usize, std::io::Error>> {
        self.project().stream.poll_write(cx, buf)
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        self.project().stream.poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        self.project().stream.poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.project().stream.poll_write(cx, buf)
    }

    fn is_write_vectored(&self) -> bool {
        self.stream.is_write_vectored()
    }
}
