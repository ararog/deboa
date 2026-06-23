#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::future::Future;

use crate::{errors::DeboaError, request::IntoRequest, response::DeboaResponse};

pub mod cache;
pub mod catcher;
pub mod cookie;
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

/// HTTP client trait
pub trait HttpClient {
    /// Execute a request
    fn execute<R>(&self, request: R) -> impl Future<Output = Result<DeboaResponse>>
    where
        R: IntoRequest;
}
