use deboa::{response::DeboaResponse, Result};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use ws_framer::{WsFrame, WsRxFramer, WsTxFramer};

pub struct WebSocket {
    stream: TokioIo<Upgraded>,
}

#[deboa::async_trait]
pub trait IntoStream {
    async fn into_stream(self) -> WebSocket;
}

#[deboa::async_trait]
impl IntoStream for DeboaResponse {
    async fn into_stream(self) -> WebSocket {
        WebSocket {
            stream: self.upgrade().await.expect("Failed to upgrade connection"),
        }
    }
}

pub trait MessageHandler: Send + Sync + 'static {
    fn on_open(&mut self);
    fn on_message(&mut self, message: String);
    fn on_close(&mut self);
}

impl WebSocket {

    pub fn split(self) -> (WebSocketReader, WebSocketWriter) {
        let (reader, writer) = tokio::io::split(self.stream);
        (WebSocketReader { stream: reader }, WebSocketWriter { stream: writer })
    }
}

pub struct WebSocketReader {
    stream: ReadHalf<TokioIo<Upgraded>>,
}

impl WebSocketReader {
    pub fn new(stream: ReadHalf<TokioIo<Upgraded>>) -> Self {
        Self { stream }
    }

    pub async fn read_message(&mut self) -> Result<Option<String>> {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);
        let stream = &mut self.stream;

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
            if let WsFrame::Text(data) = frame {
                Some(data.to_string())
            } else {
                None
            }
        } else {
            None
        };

        Ok(message)
    }
}

pub struct WebSocketWriter {
    stream: WriteHalf<TokioIo<Upgraded>>,
}

impl WebSocketWriter {
    pub fn new(stream: WriteHalf<TokioIo<Upgraded>>) -> Self {
        Self { stream }
    }

    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let result = self.stream.write_all(tx_framer.text(message)).await;
        if result.is_err() {
            return Err(deboa::errors::DeboaError::WebSocket {
                message: "Failed to send message".to_string(),
            });
        }

        Ok(())
    }
}
