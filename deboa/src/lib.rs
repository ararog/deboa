#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{errors::DeboaError, request::IntoRequest, response::DeboaResponse};
use std::{fmt::Display, future::Future};

pub mod cache;
/// Certificate module
pub mod cert;
/// Connection module
pub mod conn;
/// Cookie module
pub mod cookie;
/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.
pub mod dns;
pub mod errors;
pub mod form;
pub mod request;
pub mod response;
pub mod serde;
#[cfg(test)]
mod tests;
/// URL module
pub mod url;

/// Type alias for Result<T, DeboaError>
/// Convenience alias for handling Deboa errors throughout the library.
///
/// # Examples
///
/// ```
/// use deboa::Result;
///
/// fn example() -> Result<String> {
///     Ok("success".to_string())
/// }
/// ```
///
/// # See Also
/// - [DeboaError](crate::errors::DeboaError)
pub type Result<T> = std::result::Result<T, DeboaError>;

#[derive(PartialEq, Debug, Clone)]
/// Enum that represents the HTTP version.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 version.
/// * `Http2` - The HTTP/2 version.
/// * `Http3` - The HTTP/3 version.
pub enum HttpVersion {
    /// HTTP/1.1 version
    Http1,
    /// HTTP/2 version
    Http2,
    /// HTTP/3 version
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}

/// HTTP client trait
pub trait HttpClient {
    /// Execute a request
    fn execute<R>(&self, request: R) -> impl Future<Output = Result<DeboaResponse>>
    where
        R: IntoRequest;
}
