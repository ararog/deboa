use crate::{
    errors::WebSocketError, IntoWebSocket, Message, Result, WebSocket, WebSocketRead,
    WebSocketWrite,
};
use deboa::{
    errors::{ConnectionError, DeboaError},
    response::DeboaResponse,
};
use hyper::upgrade::{on, Upgraded};
use hyper_util::rt::TokioIo;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncReadExt as _, AsyncWrite, AsyncWriteExt as _, ReadBuf};
use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

impl IntoWebSocket for DeboaResponse {
    type UpgradedIo = TokioIo<Upgraded>;
    async fn into_websocket(self) -> deboa::Result<WebSocket<Self::UpgradedIo>> {
        let upgraded = on(self.into_inner())
            .await
            .map_err(|e| {
                DeboaError::Connection(ConnectionError::Upgrade { message: e.to_string() })
            })?;
        Ok(WebSocket::new(TokioIo::new(upgraded)))
    }
}

impl WebSocketRead for WebSocket<TokioIo<Upgraded>> {
    async fn read_message(&mut self) -> Result<Option<Message>> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);

        let bytes_read = self
            .inner
            .read(rx_framer.mut_buf())
            .await;
        if bytes_read.is_err() {
            return Err(WebSocketError::ReceiveMessage {
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

impl WebSocketWrite for &mut WebSocket<TokioIo<Upgraded>> {
    async fn write_message(&mut self, message: Message) -> Result<()> {
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
            return Err(WebSocketError::SendMessage {
                message: "Failed to send frame".to_string(),
            });
        }

        Ok(())
    }
}

impl AsyncRead for WebSocket<TokioIo<Upgraded>> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        self.project()
            .inner
            .poll_read(cx, buf)
    }
}

impl AsyncWrite for WebSocket<TokioIo<Upgraded>> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.project()
            .inner
            .poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project()
            .inner
            .poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        self.project()
            .inner
            .poll_shutdown(cx)
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.project()
            .inner
            .poll_write(cx, buf)
    }

    fn is_write_vectored(&self) -> bool {
        self.inner
            .is_write_vectored()
    }
}
