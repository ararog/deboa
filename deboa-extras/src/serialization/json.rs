use deboa::{Deboa, errors::DeboaError, response::DeboaResponse};
use serde::{Deserialize, Serialize};

pub trait JsonRequest {
    fn set_json<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError>;
}

impl JsonRequest for Deboa {
    #[cfg(feature = "json")]
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
    /// use deboa_extras::serialization::json::JsonRequest;
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
    fn set_json<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
        use http::header;

        self.edit_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string());
        let result = serde_json::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::Serialization { message: error.to_string() });
        }

        self.set_body(result.unwrap());

        Ok(self)
    }
}

pub trait JsonResponse {
    fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError>;
}

impl JsonResponse for DeboaResponse {
    fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let binding = self.raw_body();
        let body = binding.as_ref();

        let json = serde_json::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Deserialization { message: err.to_string() }),
        }
    }
}
