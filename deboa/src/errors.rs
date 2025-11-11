//! Error types for the Deboa HTTP client.
//!
//! This module contains the error types used throughout the Deboa HTTP client.
//! The main error type is `DeboaError`, which can represent various kinds of errors
//! that might occur during HTTP requests and responses.
//!
//! # Error Types
//!
//! - `DeboaError`: The main error type that can represent any error that occurs in Deboa
//! - `RequestError`: Errors that occur while building or sending HTTP requests
//! - `ResponseError`: Errors that occur while processing HTTP responses
//! - `ConnectionError`: Errors that occur during network connections
//! - `ContentError`: Errors related to content serialization/deserialization
//! - `IoError`: I/O related errors
//!
//! # Examples
//!
//! ## Handling Errors
//!
//! ```compile_fail
//! use deboa::{Deboa, request::get};
//! use deboa::errors::DeboaError;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut client = Deboa::new();
//!
//!   match get("https://example.com").and_then(|req| req.go(&mut client)) {
//!     Ok(response) => {
//!         // Handle successful response
//!     },
//!     Err(DeboaError::Connection(e)) => {
//!         // Handle connection errors
//!         eprintln!("Connection failed: {}", e);
//!     },
//!     Err(DeboaError::Request(e)) => {
//!         // Handle request errors
//!         eprintln!("Request failed: {}", e);
//!     },
//!     Err(e) => {
//!         // Handle other errors
//!         eprintln!("Error: {}", e);
//!     }
//!   }
//!   Ok(())
//! }
//! ```

use http::StatusCode;
use thiserror::Error;

/// The main error type for the Deboa HTTP client.
///
/// This enum represents all possible errors that can occur when making HTTP requests
/// with Deboa. It uses the `thiserror` crate to provide detailed error messages
/// and source information.
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
