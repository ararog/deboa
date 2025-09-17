/// A trait for cache implementations.
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
