//! # File System and I/O Module
//!
//! This module provides file system and I/O related functionality for the Deboa HTTP client.
//! It includes compression utilities and other I/O operations that support HTTP operations.
//!
//! ## Submodules
//!
//! - [`io`]: Compression utilities and I/O operations for HTTP requests
//!
//! ## Features
//!
//! - Compression algorithms for request bodies
//! - Async I/O operations
//! - Thread-safe implementations
//!
//! ## Usage
//!
//! The compression functionality can be used to reduce bandwidth usage:
//!
//! ```rust, ignore
//! use deboa::fs::io::Compressor;
//!
//! // Implement custom compression
//! struct MyCompressor;
//!
//! impl Compressor for MyCompressor {
//!     fn name(&self) -> String {
//!         "my-compression".to_string()
//!     }
//!
//!     async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
//!         // Compression logic here
//!     }
//! }
//! ```

pub mod io;
