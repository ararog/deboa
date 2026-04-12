//! # Deboa - Core API Documentation
//!
//! Hello, and welcome to the core Deboa API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//!
//! Depend on `deboa` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = "0.1.0"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ``` rust,ignore
//! use deboa::{Client, Result, errors::DeboaError, request::DeboaRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut client = Client::builder()
//!         .build();
//!
//!     let response = DeboaRequest::get("https://httpbin.org/get")?
//!         .send_with(&mut client)
//!         .await?;
//!
//!     println!("Response: {:#?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! To avoid compiling unused dependencies, Deboa feature-gates optional
//! functionality, some enabled by default:
//!
//! | Feature             | Default? | Description                                             |
//! |---------------------|----------|---------------------------------------------------------|
//! | `tokio_rt`          | Yes      | Support tokio runtime (enabled by default).             |
//! | `smol_rt`           | No       | Support smol runtime.                                   |
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.1.0", features = ["tokio_rt", "http2", "tokio-rust-tls", "default-rustls-provider", "default-rustls-verifier"] }
//! ```
//!

use crate::{errors::DeboaError, request::IntoRequest, response::DeboaResponse};

pub mod cache;
pub mod catcher;
pub mod cookie;
pub mod errors;
pub mod form;
pub mod request;
pub mod response;
pub mod serde;
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

pub trait HttpClient {
    async fn execute<R>(&self, request: R) -> Result<DeboaResponse>
    where
        R: IntoRequest;
}
