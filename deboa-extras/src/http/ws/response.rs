use std::future::Future;

use deboa::{errors::DeboaError, response::DeboaResponse, Result};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use websocket_sans_io::{FrameInfo, Opcode, WebsocketFrameEvent};

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
        let mut frame_encoder = websocket_sans_io::WebsocketFrameEncoder::new();
        let header = frame_encoder.start_frame(&FrameInfo {
            opcode: Opcode::Text,
            payload_length: message.len() as websocket_sans_io::PayloadLength,
            mask: Some(1234u32.to_be_bytes()),
            fin: true,
            reserved: 0,
        });
        if let Err(e) = self.stream.write(&header).await {
            return Err(DeboaError::WebSocket {
                message: e.to_string(),
            });
        }
        Ok(())
    }

    pub async fn poll_message<F, Fut>(&mut self, mut on_event: F) -> Result<()>
    where
        F: FnMut(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<()>> + Send + 'static,
    {
        let mut frame_decoder = websocket_sans_io::WebsocketFrameDecoder::new();
        let mut result = Vec::<u8>::new();
        let mut buf = [0u8; 1024];
        'read_loop: loop {
            let n = self.stream.read(&mut buf).await.unwrap();
            let mut processed_offset = 0;
            'decode_loop: loop {
                let unprocessed_part_of_buf = &mut buf[processed_offset..n];
                let ret = frame_decoder.add_data(unprocessed_part_of_buf).unwrap();
                processed_offset += ret.consumed_bytes;

                if ret.event.is_none() && ret.consumed_bytes == 0 {
                    break 'decode_loop;
                }

                match ret.event {
                    Some(WebsocketFrameEvent::PayloadChunk {
                        original_opcode: Opcode::Text,
                    }) => {
                        result.extend_from_slice(&unprocessed_part_of_buf[0..ret.consumed_bytes]);
                    }

                    Some(WebsocketFrameEvent::End {
                        frame_info: FrameInfo { fin: true, .. },
                        original_opcode: Opcode::Text,
                    }) => {
                        break 'read_loop;
                    }

                    _ => (),
                }

                on_event(String::from_utf8_lossy(&result).to_string()).await?;
            }
        }
        Ok(())
    }
}
