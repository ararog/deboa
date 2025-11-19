//! # Serialization/Deserialization Module
//!
//! This module provides traits and utilities for handling HTTP request and response body
//! serialization and deserialization. It enables automatic conversion between Rust types
//! and various data formats like JSON, form data, and other content types.
//!
//! ## Key Components
//!
//! - [`RequestBody`]: Trait for serializing request bodies into bytes
//! - [`ResponseBody`]: Trait for deserializing response bodies from bytes
//! - Content type registration and management
//! - Format-agnostic serialization interface
//!
//! ## Features
//!
//! - **Format Agnostic**: Support for multiple serialization formats
//! - **Content Type Management**: Automatic content-type header handling
//! - **Type Safety**: Compile-time guarantees for serializable types
//! - **Error Handling**: Comprehensive error reporting for serialization failures
//! - **Extensibility**: Easy to add new serialization formats
//!
//! ## Usage
//!
//! ### Implementing Custom Request Body Serializer
//!
//! ```rust, ignore
//! use deboa::client::serde::RequestBody;
//! use deboa::{request::DeboaRequest, Result};
//! use serde::Serialize;
//!
//! struct JsonBody;
//!
//! impl RequestBody for JsonBody {
//!     fn register_content_type(&self, request: &mut DeboaRequest) {
//!         request.set_content_type("application/json");
//!     }
//!
//!     fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>> {
//!         serde_json::to_vec(&value).map_err(Into::into)
//!     }
//! }
//! ```
//!
//! ### Implementing Custom Response Body Deserializer
//!
//! ```rust, ignore
//! use deboa::client::serde::ResponseBody;
//! use deboa::Result;
//! use serde::Deserialize;
//!
//! struct JsonBody;
//!
//! impl ResponseBody for JsonBody {
//!     fn deserialize<T: for<'de> Deserialize<'de>>(&self, bytes: &[u8]) -> Result<T> {
//!         serde_json::from_slice(bytes).map_err(Into::into)
//!     }
//! }
//! ```
//!
//! ## Integration with HTTP Client
//!
//! The serialization traits are used internally by the Deboa client to handle:
//! - Automatic JSON serialization for request bodies
//! - JSON deserialization for response bodies
//! - Form data encoding and decoding
//! - Custom content type handling

use crate::{request::DeboaRequest, Result};
use serde::{Deserialize, Serialize};

/// Trait that represents request body serialization capabilities.
///
/// This trait defines the interface for converting Rust types into HTTP request bodies.
/// Implementations handle different serialization formats like JSON, XML, form data, etc.
///
/// # Requirements
///
/// Implementations must:
/// - Register the appropriate content type on the request
/// - Serialize the given value into a byte vector
/// - Handle serialization errors appropriately
///
/// # Examples
///
/// ## JSON Serializer
///
/// ```rust, ignore
/// use deboa::client::serde::RequestBody;
/// use deboa::{request::DeboaRequest, Result};
/// use serde::Serialize;
///
/// struct JsonSerializer;
///
/// impl RequestBody for JsonSerializer {
///     fn register_content_type(&self, request: &mut DeboaRequest) {
///         request.set_content_type("application/json");
///     }
///
///     fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>> {
///         serde_json::to_vec(&value)
///             .map_err(|e| DeboaError::SerializationError(e.to_string()))
///     }
/// }
/// ```
///
/// ## Form URL-encoded Serializer
///
/// ```rust, ignore
/// use deboa::client::serde::RequestBody;
/// use deboa::{request::DeboaRequest, Result};
/// use serde::Serialize;
///
/// struct FormSerializer;
///
/// impl RequestBody for FormSerializer {
///     fn register_content_type(&self, request: &mut DeboaRequest) {
///         request.set_content_type("application/x-www-form-urlencoded");
///     }
///
///     fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>> {
///         serde_urlencoded::to_string(&value)
///             .map(|s| s.into_bytes())
///             .map_err(|e| DeboaError::SerializationError(e.to_string()))
///     }
/// }
/// ```
pub trait RequestBody {
    /// Register the content type on the Deboa instance
    ///
    /// This method is called to set the appropriate Content-Type header
    /// for the serialized data format.
    ///
    /// # Arguments
    ///
    /// * `request` - A mutable reference to the DeboaRequest instance
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// fn register_content_type(&self, request: &mut DeboaRequest) {
    ///     request.set_content_type("application/json");
    /// }
    /// ```
    fn register_content_type(&self, request: &mut DeboaRequest) -> ();

    /// Serialize the request body
    ///
    /// Converts the given value into a byte vector suitable for transmission
    /// as an HTTP request body.
    ///
    /// # Arguments
    ///
    /// * `value` - The request body to serialize
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>>` - The serialized request body as bytes
    ///
    /// # Errors
    ///
    /// Returns an error if serialization fails, such as when the value
    /// cannot be represented in the target format.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>> {
    ///     serde_json::to_vec(&value)
    ///         .map_err(|e| DeboaError::SerializationError(e.to_string()))
    /// }
    /// ```
    fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>>;
}

/// Trait that represents response body deserialization capabilities.
///
/// This trait defines the interface for converting HTTP response bodies
/// from bytes into Rust types. Implementations handle different deserialization
/// formats like JSON, XML, text, etc.
///
/// # Requirements
///
/// Implementations must:
/// - Parse bytes into the target Rust type
/// - Handle deserialization errors appropriately
/// - Support the target data format
///
/// # Examples
///
/// ## JSON Deserializer
///
/// ```rust, ignore
/// use deboa::client::serde::ResponseBody;
/// use deboa::Result;
/// use serde::Deserialize;
///
/// struct JsonBody;
///
/// impl ResponseBody for JsonBody {
///     fn deserialize<T: for<'de> Deserialize<'de>>(&self, bytes: &[u8]) -> Result<T> {
///         serde_json::from_slice(bytes)
///             .map_err(|e| DeboaError::DeserializationError(e.to_string()))
///     }
/// }
/// ```
///
/// ## Text Deserializer
///
/// ```rust, ignore
/// use deboa::client::serde::ResponseBody;
/// use deboa::Result;
/// use std::str;
///
/// struct TextBody;
///
/// impl ResponseBody for TextBody {
///     fn deserialize<T: for<'de> Deserialize<'de>>(&self, bytes: &[u8]) -> Result<T> {
///         let text = str::from_utf8(bytes)
///             .map_err(|e| DeboaError::DeserializationError(e.to_string()))?;
///         
///         // For simple string types
///         if std::any::TypeId::of::<T>() == std::any::TypeId::of::<String>() {
///             // SAFETY: We've verified the type
///             Ok(unsafe { std::mem::transmute_copy(&text) })
///         } else {
///             Err(DeboaError::DeserializationError("Unsupported type".to_string()))
///         }
///     }
/// }
/// ```
pub trait ResponseBody {
    /// Deserialize the response body
    ///
    /// Converts the given byte vector into the target Rust type.
    ///
    /// # Arguments
    ///
    /// * `value` - The response body bytes to deserialize
    ///
    /// # Returns
    ///
    /// * `Result<T>` - The deserialized response body
    ///
    /// # Errors
    ///
    /// Returns an error if deserialization fails, such as when:
    /// - The bytes cannot be parsed as the target format
    /// - The data structure doesn't match the expected type
    /// - The bytes contain invalid UTF-8 for text formats
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to deserialize into, must implement `Deserialize`
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// fn deserialize<T: for<'de> Deserialize<'de>>(&self, value: Vec<u8>) -> Result<T> {
    ///     serde_json::from_slice(&value)
    ///         .map_err(|e| DeboaError::DeserializationError(e.to_string()))
    /// }
    /// ```
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, value: Vec<u8>) -> Result<T>;
}
