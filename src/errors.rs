use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DeboaError {
    #[error("Could not connect to {host}: {message}")]
    ConnectionError { host: String, message: String },

    #[error("Failed to send request: {method} {host}/{path}: {message}")]
    RequestError { host: String, path: String, method: String, message: String },

    #[error("Failed to serialize data: {message}")]
    SerializationError { message: String },

    #[error("Failed to deserialize data: {message}")]
    DeserializationError { message: String },
}
