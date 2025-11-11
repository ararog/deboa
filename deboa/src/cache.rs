//! Caching functionality for the Deboa HTTP client.
//!
//! This module provides the `DeboaCache` trait for implementing custom cache
//! backends. Implement this trait to provide your own caching mechanism for
//! HTTP responses.
//!
//! # Features
//!
//! - Simple key-value interface
//! - Thread-safe operations
//! - Optional TTL support (can be implemented by the concrete type)
//!
//! # Examples
//!
//! ## Implementing a custom cache
//!
//! ```no_run
//! use deboa::cache::DeboaCache;
//! use std::collections::HashMap;
//! use std::sync::{Arc, RwLock};
//!
//! #[derive(Default)]
//! struct MemoryCache {
//!     store: Arc<RwLock<HashMap<String, String>>>,
//! }
//!
//! impl DeboaCache for MemoryCache {
//!     fn get(&self, key: &str) -> Option<String> {
//!         self.store.read().unwrap().get(key).cloned()
//!     }
//!
//!     fn set(&self, key: &str, value: &str) {
//!         self.store.write().unwrap().insert(key.to_string(), value.to_string());
//!     }
//!
//!     fn delete(&self, key: &str) {
//!         self.store.write().unwrap().remove(key);
//!     }
//! }
//! ```
//!
//! ## Using the cache with Deboa
//!
//! ```no_run
//! # use deboa::{Deboa, cache::DeboaCache};
//! # struct MyCache;
//! # impl DeboaCache for MyCache {
//! #   fn get(&self, _: &str) -> Option<String> { None }
//! #   fn set(&self, _: &str, _: &str) {}
//! #   fn delete(&self, _: &str) {}
//! # }
//! #
//! let cache = MyCache; // Your cache implementation
//! // let client = Deboa::builder().cache(Box::new(cache)).build();
//! ```

/// A trait defining the interface for cache implementations.
///
/// Implement this trait to provide custom caching behavior for HTTP responses.
/// The cache is responsible for storing and retrieving responses based on their
/// cache keys.
///
/// # Thread Safety
///
/// Implementations must be thread-safe as they may be accessed concurrently
/// from multiple threads.
pub trait DeboaCache {
    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to get the value from.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The value if it exists, None otherwise.
    ///
    fn get(&self, key: &str) -> Option<String>;
    /// Set a value in the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to set the value for.
    /// * `value` - The value to set.
    ///
    fn set(&self, key: &str, value: &str);
    /// Delete a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to delete the value for.
    ///
    fn delete(&self, key: &str);
}
