//! # Compression I/O Module
//!
//! This module provides compression functionality for HTTP requests and responses.
//! It defines a `Compressor` trait that allows for different compression algorithms
//! to be implemented and used with the Deboa HTTP client.
//!
//! ## Purpose
//!
//! The compression module enables automatic compression of request bodies to reduce
//! bandwidth usage and improve performance when sending large payloads.
//!
//! ## Usage
//!
//! Implement the `Compressor` trait for your desired compression algorithm:
//!
//! ```rust, ignore
//! use deboa::fs::io::Compressor;
//! use async_trait::async_trait;
//! use bytes::Bytes;
//! use deboa::{request::DeboaRequest, Result};
//!
//! #[async_trait]
//! impl Compressor for MyCompressor {
//!     fn name(&self) -> String {
//!         "my-compression".to_string()
//!     }
//!
//!     async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
//!         // Implement compression logic here
//!         Ok(Bytes::from("compressed-data"))
//!     }
//! }
//! ```
//!
//! ## Supported Features
//!
//! - Async compression operations
//! - Thread-safe implementations (`Send + Sync`)
//! - Custom compression algorithms
//! - Integration with HTTP request pipeline

#![allow(unused_variables)]
use async_trait::async_trait;
use bytes::Bytes;

use crate::{request::DeboaRequest, response::DeboaResponse, Result};

/// Trait that represents a compression algorithm for HTTP request bodies.
///
/// This trait defines the interface that all compression implementations must follow.
/// It enables automatic compression of request payloads to reduce bandwidth usage
/// and improve transfer speeds.
///
/// # Requirements
///
/// Implementations must be:
/// - `Send`: Safe to transfer across thread boundaries
/// - `Sync`: Safe to share between threads
/// - `'static`: Valid for the entire lifetime of the program
///
/// # Examples
///
/// ```rust, ignore
/// use deboa::fs::io::Compressor;
/// use async_trait::async_trait;
/// use bytes::Bytes;
/// use deboa::{request::DeboaRequest, Result};
///
/// struct GzipCompressor;
///
/// #[async_trait]
/// impl Compressor for GzipCompressor {
///     fn name(&self) -> String {
///         "gzip".to_string()
///     }
///
///     async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
///         // Implement gzip compression
///         Ok(request.body().clone())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Compressor: Send + Sync + 'static {
    /// This method returns the name of encoding for this compressor.
    ///
    /// # Returns
    ///
    /// * `String` - The name of the encoding.
    ///
    fn name(&self) -> String;
    /// This method compress the body of the request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `Result<Bytes>` - The compressed body of the request.
    ///
    async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes>;
}

#[async_trait]
impl<T: Compressor> Compressor for Box<T> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    async fn compress_body(&self, request: &DeboaRequest) -> Result<Bytes> {
        self.as_ref()
            .compress_body(request)
            .await
    }
}

/// Trait that represents the decompressor.
#[async_trait]
pub trait Decompressor: Send + Sync + 'static {
    /// This method register the encoding of the response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    fn name(&self) -> String;
    /// This method decompress the body of the response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - The decompressed body of the response.
    ///
    async fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl<T: Decompressor> Decompressor for Box<T> {
    fn name(&self) -> String {
        self.as_ref().name()
    }

    async fn decompress_body(&self, response: &mut DeboaResponse) -> Result<()> {
        self.as_ref()
            .decompress_body(response)
            .await
    }
}
