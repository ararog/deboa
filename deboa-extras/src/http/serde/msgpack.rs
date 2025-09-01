pub struct MsgPackBody;

impl RequestBody for MsgPackBody {
    /// Allow set msgpack body at any time.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be serialized, it must be a struct that implements Serialize.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Post {
    ///     id: u32,
    ///     title: String,
    ///     body: String,
    /// }
    /// /*
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_msgpack(Post { id: 1, title: "title".to_string(), body: "body".to_string() })?.post("/posts").await?;
    ///   Ok(())
    /// }
    /// */
    /// ```
    ///
    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        let result = rmp_serde::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::SerializationError { message: error.to_string() });
        }

        self.body = Some(result.unwrap());

        Ok(self)
    }
}

impl ResponseBody for MsgPackBody {
    fn deserialize<T: Deserialize<'_>>(&self, body: Vec<u8>) -> Result<T, DeboaError> {
        let binding = body;
        let body = binding.as_ref();

        let json = serde_json::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Deserialization { message: err.to_string() }),
        }
    }
}
