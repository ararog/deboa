use std::future::Future;

use deboa::{response::DeboaResponse, Result};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ws_framer::{WsRxFramer, WsTxFramer};

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

impl WebSocket {
    pub async fn send_message(&mut self, message: &str) -> Result<()> {
        let mut tx_buf = vec![0; 10240];
        let mut tx_framer = WsTxFramer::new(true, &mut tx_buf);

        let mut buf = Vec::new();
        buf.extend_from_slice(tx_framer.text(message));
        self.stream.write_all(&buf).await;
        Ok(())
    }

    pub async fn poll_message<F, Fut>(&mut self, mut on_event: F) -> Result<()>
    where
        F: FnMut(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let mut rx_buf = vec![0; 10240];
        let mut rx_framer = WsRxFramer::new(&mut rx_buf);
        loop {
            let read_n = self.stream.read(rx_framer.mut_buf()).await;
            if read_n.is_err() {
                break;
            }

            rx_framer.revolve_write_offset(read_n.unwrap());
            let res = rx_framer.process_data();
            if res.is_some() {
                on_event(String::from_utf8_lossy(res.unwrap().data()).to_string()).await;
            }
        }

        Ok(())
    }
}
