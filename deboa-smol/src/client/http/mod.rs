//! Core HTTP client implementation for Deboa.
//!
//! This module provides the main HTTP client functionality, including connection management
//! and request/response handling. It's the central component for making HTTP requests
//! in the Deboa library.

/// Connection management functionality for the Deboa HTTP client.
pub mod conn;

#[cfg(feature = "http1")]
pub(crate) mod http1;
#[cfg(feature = "http2")]
pub(crate) mod http2;
#[cfg(feature = "http3")]
pub(crate) mod http3;
