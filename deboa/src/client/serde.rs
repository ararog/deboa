use crate::{errors::DeboaError, request::DeboaRequest};
use serde::{Deserialize, Serialize};

pub trait RequestBody {
    /// Register the content type on the Deboa instance
    ///
    /// # Arguments
    ///
    /// * `request` - A mutable reference to the DeboaRequest instance
    ///
    fn register_content_type(&self, request: &mut DeboaRequest) -> ();
    /// Serialize the request body
    ///
    /// # Arguments
    ///
    /// * `value` - The request body to serialize
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, DeboaError>` - The serialized request body
    ///
    fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>, DeboaError>;
}

pub trait ResponseBody {
    /// Deserialize the response body
    ///
    /// # Arguments
    ///
    /// * `value` - The response body to deserialize
    ///
    /// # Returns
    ///
    /// * `Result<T, DeboaError>` - The deserialized response body
    ///
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, value: Vec<u8>) -> Result<T, DeboaError>;
}
