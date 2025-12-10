//! Core HTTP client implementation for Deboa.
//!
//! This module provides the main HTTP client functionality, including connection management
//! and request/response handling. It's the central component for making HTTP requests
//! in the Deboa library.

/// Connection management functionality for the Deboa HTTP client.
pub mod conn;

/// Serialization and deserialization traits for the Deboa HTTP client.
pub mod serde;
