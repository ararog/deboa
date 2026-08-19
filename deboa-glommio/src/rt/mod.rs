//! # Runtime Abstraction Layer
//!
//! This module provides runtime-specific implementations for different async runtimes.
//! It allows the library to work with multiple async runtimes through feature flags.
//!
//! ## Usage
//!
//! The appropriate runtime module will be automatically selected based on the enabled features.
//! You don't need to interact with this module directly in most cases.
//!
//! ## Features
//!
//! Enable the corresponding feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.deboa]
//! version = "0.0.8"
//! features = ["http1", "rust-tls"]
//! ```

/// Stream module for runtime-specific stream implementations.
pub mod stream;

/// Executor module for HTTP/2 operations
#[cfg(feature = "http2")]
pub(crate) mod executor;
