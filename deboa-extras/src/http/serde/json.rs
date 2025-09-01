use deboa::{
    errors::DeboaError,
    http::serde::{RequestBody, ResponseBody},
};
use serde::{Deserialize, Serialize};

pub struct JsonBody;

impl RequestBody for JsonBody {
    /// Allow set json body at any time.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized, it must be a struct that implements Serialize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, errors::DeboaError};
    /// use deboa_extras::http::serde::json::JsonBody;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Post {
    ///     id: u32,
    ///     title: String,
    ///     body: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_json(Post { id: 1, title: "title".to_string(), body: "body".to_string() })?.post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        let result = serde_json::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::Serialization { message: error.to_string() });
        }

        Ok(result.unwrap())
    }
}

impl ResponseBody for JsonBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T, DeboaError> {
        let binding = body;
        let body = binding.as_ref();

        let json = serde_json::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Deserialization { message: err.to_string() }),
        }
    }
}
