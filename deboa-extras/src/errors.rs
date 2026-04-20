use thiserror::Error;

/// Deboa extras errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum DeboaExtrasError {
    /// Websocket error
    #[error("Websocket error: {0}")]
    WebSocket(#[from] WebSocketError),

    /// SSE error
    #[error("SSE error: {0}")]
    SSE(#[from] SSEError),
}

/// WebSocket errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum WebSocketError {
    /// Failed to send message
    #[error("Failed to send message: {message}")]
    SendMessage {
        /// The error message
        message: String,
    },

    /// Failed to receive message
    #[error("Failed to receive message: {message}")]
    ReceiveMessage {
        /// The error message
        message: String,
    },
}

/// SSE errors
#[derive(Debug, Clone, Error, PartialEq)]
pub enum SSEError {
    /// Failed to receive event
    #[error("Failed to receive event: {message}")]
    ReceiveEvent {
        /// The error message
        message: String,
    },
}
