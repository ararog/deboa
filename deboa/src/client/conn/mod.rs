//! Connection management for the Deboa HTTP client.
//!
//! This module provides the building blocks for managing HTTP connections,
//! including connection pooling and protocol-specific implementations.
//!
//! # Architecture
//!
//! - [`http`]: Core HTTP protocol implementations (HTTP/1.1, HTTP/2)
//! - [`pool`]: Connection pooling for efficient request handling
//!
//! # Features
//!
//! - Automatic connection pooling
//! - Protocol negotiation (HTTP/1.1, HTTP/2)
//! - Connection lifecycle management
//! - Thread-safe connection handling
//! ```

/// HTTP protocol implementations.
///
/// This module contains the core HTTP protocol implementations, including:
/// - HTTP/1.1 support
/// - HTTP/2 support (when enabled)
/// - Connection management
/// - Request/response handling
///
/// # Features
///
/// - `http1`: Enables HTTP/1.1 support
/// - `http2`: Enables HTTP/2 support (requires TLS)
pub mod http;

/// Connection pooling for efficient HTTP connections.
///
/// This module provides connection pooling functionality to reuse connections
/// across multiple requests, reducing latency and resource usage.
///
/// # Features
///
/// - Automatic connection reuse
/// - Connection lifecycle management
/// - Thread-safe operation
/// - Configurable pool size (coming soon)
pub mod pool;

/// Internal stream handling utilities for connection establishment.
/// Provides low-level connection creation functions for both secure and insecure connections.
/// Used internally by the HTTP connection implementations.
///
/// # Modules
///
/// - `plain_connection`: Creates plain (non-TLS) TCP connections
/// - `tls_connection`: Creates TLS-encrypted connections with optional client certificates
///
/// # Examples
///
/// ```compile_fail, rust
/// use deboa::client::conn::stream::{plain_connection, tls_connection};
///
/// // Create a plain TCP connection
/// let stream = plain_connection("example.com:80").await?;
///
/// // Create a TLS connection
/// let stream = tls_connection("example.com:443", None).await?;
/// ```
pub(crate) mod stream;
