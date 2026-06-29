use crate::{
    errors::{DeboaExtrasError, WebSocketError},
    ws::protocol::Message,
};
use hyper::upgrade::Upgraded;
use pin_project_lite::pin_project;
use smol::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use smol_hyper::rt::FuturesIo;
use std::{
    future::Future,
    io,
    pin::Pin,
    task::{Context, Poll},
};
use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

pub type UpgradedIo = FuturesIo<Upgraded>;

pub trait DeboaWebSocket {
    type Stream;

    fn new(stream: Self::Stream) -> Self;
    fn read_message(&mut self) -> impl Future<Output = Result<Option<Message>, DeboaExtrasError>>;
    fn write_message(
        &mut self,
        message: Message,
    ) -> impl Future<Output = Result<(), DeboaExtrasError>>;
    fn send_close(
        &mut self,
        code: u16,
        reason: &str,
    ) -> impl Future<Output = Result<(), DeboaExtrasError>>;
    fn send_text(&mut self, message: &str) -> impl Future<Output = Result<(), DeboaExtrasError>>;
    fn send_binary(&mut self, message: &[u8])
        -> impl Future<Output = Result<(), DeboaExtrasError>>;
    fn send_ping(&mut self, message: &[u8]) -> impl Future<Output = Result<(), DeboaExtrasError>>;
    fn send_pong(&mut self, message: &[u8]) -> impl Future<Output = Result<(), DeboaExtrasError>>;
}

pin_project! {
    /// WebSocket struct
    pub struct WebSocket<T>
    {
        #[pin]
        stream: T,
    }
}

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

    /// Reads a message from the WebSocket.
    ///
    /// # Returns
    ///
    /// A Result containing an Option<Message> or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// while let Some(message) = websocket.read_message().await {
    ///     println!("message: {}", message);
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn read_message(&mut self) -> Result<Option<Message>, DeboaExtrasError> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);

        let bytes_read = self
            .stream
            .read(rx_framer.mut_buf())
            .await;
        if bytes_read.is_err() {
            return Err(DeboaExtrasError::WebSocket(WebSocketError::ReceiveMessage {
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

    /// Writes a message to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to write.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket
    ///   .write_message(protocol::Message::Text(message.to_string()))
    ///   .await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    ///
    async fn write_message(&mut self, message: Message) -> Result<(), DeboaExtrasError> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let result = match message {
            Message::Text(data) => {
                self.write_all(tx_framer.frame(WsFrame::Text(&data)))
                    .await
            }
            Message::Binary(data) => {
                self.write_all(tx_framer.frame(WsFrame::Binary(&data)))
                    .await
            }
            Message::Close(code, reason) => {
                self.write_all(tx_framer.frame(WsFrame::Close(code, &reason)))
                    .await
            }
            Message::Ping(data) => {
                self.write_all(tx_framer.frame(WsFrame::Ping(&data)))
                    .await
            }
            _ => Ok(()),
        };

        if result.is_err() {
            return Err(DeboaExtrasError::WebSocket(WebSocketError::SendMessage {
                message: "Failed to send frame".to_string(),
            }));
        }

        Ok(())
    }

    /// Sends a close frame to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `code` - The close code.
    /// * `reason` - The close reason.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket.send_close(1000, "Goodbye").await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn send_close(&mut self, code: u16, reason: &str) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Close(code, reason.to_string()))
            .await
    }

    /// Sends a text frame to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The text message to send.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket.send_text("Hello").await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn send_text(&mut self, message: &str) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Text(message.to_string()))
            .await
    }

    /// Sends a binary frame to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The binary message to send.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket.send_binary(&[0x00, 0x01, 0x02]).await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn send_binary(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Binary(message.to_vec()))
            .await
    }

    /// Sends a ping frame to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The ping message to send.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket.send_ping(&[0x00, 0x01, 0x02]).await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn send_ping(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Ping(message.to_vec()))
            .await
    }

    /// Sends a pong frame to the WebSocket.
    ///
    /// # Arguments
    ///
    /// * `message` - The pong message to send.
    ///
    /// # Returns
    ///
    /// A Result indicating success or a DeboaExtrasError.
    ///
    /// # Examples
    ///
    /// ```rust, compile_fail
    /// let result = websocket.send_pong(&[0x00, 0x01, 0x02]).await;
    /// if result.is_err() {
    ///     output.send(Event::Disconnected).await;
    ///     break;
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function may panic if the WebSocket frame processing fails.
    ///
    async fn send_pong(&mut self, message: &[u8]) -> Result<(), DeboaExtrasError> {
        self.write_message(Message::Pong(message.to_vec()))
            .await
    }
}

impl<T> AsyncRead for WebSocket<FuturesIo<T>>
where
    T: hyper::rt::Read,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Poll::Ready(Ok(0))
    }
}

impl<T> AsyncWrite for WebSocket<FuturesIo<T>>
where
    T: hyper::rt::Write,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        hyper::rt::Write::poll_write(
            self.project()
                .stream
                .get_pin_mut(),
            cx,
            buf,
        )
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        hyper::rt::Write::poll_flush(
            self.project()
                .stream
                .get_pin_mut(),
            cx,
        )
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        hyper::rt::Write::poll_shutdown(
            self.project()
                .stream
                .get_pin_mut(),
            cx,
        )
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        hyper::rt::Write::poll_write_vectored(
            self.project()
                .stream
                .get_pin_mut(),
            cx,
            bufs,
        )
    }
}
