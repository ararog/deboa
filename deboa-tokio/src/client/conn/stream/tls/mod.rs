#[cfg(feature = "native-tls")]
mod native;

#[cfg(feature = "rust-tls")]
/// Internal stream handling utilities for connection establishment.
/// Provides low-level connection creation function for secure connections .
/// Used internally by the HTTP connection implementations.
///
/// # Modules
///
/// - `tls_connection`: Creates TLS-encrypted connections with optional client certificates
///
/// # Examples
///
/// ```compile_fail, rust
/// use deboa::client::conn::stream::tls_connection;
///
/// // Create a TLS connection
/// let stream = tls_connection("example.com:443", None).await?;
/// ```
mod rustls;

#[cfg(feature = "rust-tls")]
pub(crate) use rustls::*;

#[cfg(feature = "native-tls")]
pub(crate) use native::*;
