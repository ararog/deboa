pub trait MsgpackRequest {
    fn set_msgpack<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError>;
}

impl MsgpackRequest for Deboa {
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
    fn set_msgpack<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
        let result = rmp_serde::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::SerializationError { message: error.to_string() });
        }

        self.body = Some(result.unwrap());

        Ok(self)
    }
}
