use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum WebSocketError {
    #[error("Error receiving message: {message}")]
    ReceiveMessage {
        /// Error message
        message: String,
    },

    #[error("Error sending message: {message}")]
    SendMessage {
        /// Error message
        message: String,
    },
}
