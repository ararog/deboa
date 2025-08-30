use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum DeboaError {
    #[error("Could not connect to {host}: {message}")]
    Connection { host: String, message: String },

    #[error("Failed to send request: {method} {host}/{path}: {message}")]
    Request {
        host: String,
        path: String,
        method: String,
        message: String,
    },

    #[error("Failed to serialize data: {message}")]
    Serialization { message: String },

    #[error("Failed to deserialize data: {message}")]
    Deserialization { message: String },

    #[error("Failed to parse url: {message}")]
    UrlParse { message: String },
}
