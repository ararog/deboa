use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq)]
pub enum DeboaError {
    #[error("Invalid cookie header: {message}")]
    Cookie { message: String },

    #[error("Invalid client certificate: {message}")]
    ClientCert { message: String },

    #[error("Invalid header: {message}")]
    Header { message: String },

    #[error("Connection error: {0}")]
    Connection(#[from] ConnectionError),

    #[error("Request error: {0}")]
    Request(#[from] RequestError),

    #[error("Response error: {0}")]
    Response(#[from] ResponseError),

    #[error("Content error: {0}")]
    Content(#[from] ContentError),

    #[error("Io error: {0}")]
    Io(#[from] IoError),
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum RequestError {
    #[error("Failed to send request: {method} {url}: {message}")]
    Send {
        url: String,
        method: String,
        message: String,
    },

    #[error("Failed to prepare request: {message}")]
    Prepare { message: String },

    #[error("Failed to parse url: {message}")]
    UrlParse { message: String },

    #[error("Failed to parse method: {message}")]
    MethodParse { message: String },
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ResponseError {
    #[error("Failed to receive response: {status_code}: {message}")]
    Receive {
        status_code: StatusCode,
        message: String,
    },

    #[error("Failed to process response: {message}")]
    Process { message: String },
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ConnectionError {
    #[error("Tcp connection error: {host} {message}")]
    Tcp { host: String, message: String },

    #[error("Tls connection error: {host} {message}")]
    Tls { host: String, message: String },

    #[error("Connection handshake error: {host} {message}")]
    Handshake { host: String, message: String },

    #[error("Connection upgrade error: {message}")]
    Upgrade { message: String },

    #[error("Unsupported scheme: {message}")]
    UnsupportedScheme { message: String },
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum ContentError {
    #[error("Failed to serialize data: {message}")]
    Serialization { message: String },

    #[error("Failed to deserialize data: {message}")]
    Deserialization { message: String },
}

#[derive(Debug, Clone, Error, PartialEq)]
pub enum IoError {
    #[error("Failed to write file: {message}")]
    File { message: String },

    #[error("Failed to compress data: {message}")]
    Compress { message: String },

    #[error("Failed to decompress data: {message}")]
    Decompress { message: String },

    #[error("Failed to write to stdout: {message}")]
    Stdout { message: String },

    #[error("Failed to write to stderr: {message}")]
    Stderr { message: String },

    #[error("Failed to read from stdin: {message}")]
    Stdin { message: String },
}
