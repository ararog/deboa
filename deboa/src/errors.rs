use http::StatusCode;
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

    #[error("Failed to receive response: {status_code}: {message}")]
    Response { status_code: StatusCode, message: String },

    #[error("Failed to process response: {message}")]
    ProcessResponse { message: String },

    #[error("Unsupported scheme: {message}")]
    UnsupportedScheme { message: String },

    #[error("Failed to serialize data: {message}")]
    Serialization { message: String },

    #[error("Failed to deserialize data: {message}")]
    Deserialization { message: String },

    #[error("Failed to parse url: {message}")]
    UrlParse { message: String },

    #[error("Failed to compress data: {message}")]
    Compress { message: String },

    #[error("Failed to decompress data: {message}")]
    Decompress { message: String },

    #[error("Failed to write file: {message}")]
    Io { message: String },
}
