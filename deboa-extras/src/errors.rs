use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum DeboaExtrasError {
    #[error("Websocket error: {0}")]
    WebSocket(#[from] WebSocketError),

    #[error("SSE error: {0}")]
    SSE(#[from] SSEError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum WebSocketError {
    #[error("Failed to send message: {message}")]
    SendMessage { message: String },

    #[error("Failed to receive message: {message}")]
    ReceiveMessage { message: String },
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum SSEError {
    #[error("Failed to receive event: {message}")]
    ReceiveEvent { message: String },
}
