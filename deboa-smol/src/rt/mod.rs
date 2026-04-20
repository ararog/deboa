#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
pub(crate) mod tls;

/// Stream module for network operations
pub mod stream;

#[cfg(feature = "http2")]
pub(crate) mod executor;
