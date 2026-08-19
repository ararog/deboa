//! TLS transport implementations for the Deboa HTTP client.
//!
//! This module provides TLS functionality for secure HTTP connections.
//! It supports both native-tls and rustls backends.

#[cfg(feature = "native-tls")]
pub mod native;
#[cfg(feature = "rust-tls")]
pub mod rustls;
