//! # Runtime Abstraction Layer
//!
//! This module provides runtime-specific implementations for different async runtimes.
//! It allows the library to work with multiple async runtimes through feature flags.
//!
//! ## Available Runtimes
//!
//! - `tokio-rt`: Enables support for the Tokio runtime
//! - `smol-rt`: Enables support for the smol runtime
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
//! features = ["tokio-rt"]  # or "smol-rt"
//! ```

#[cfg(feature = "tokio-rt")]
pub mod tokio;

#[cfg(feature = "smol-rt")]
pub mod smol;
