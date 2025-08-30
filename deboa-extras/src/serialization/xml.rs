pub trait XmlRequest {
    fn set_xml<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError>;
}

impl XmlRequest for Deboa {
    #[cfg(feature = "xml")]
    /// Allow set xml body at any time.
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
    /// use http::header;
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
    ///   api.set_xml(Post { id: 1, title: "title".to_string(), body: "body".to_string() })?.post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    fn set_xml<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
        self.edit_header(header::CONTENT_TYPE, APPLICATION_XML.to_string());
        self.edit_header(header::ACCEPT, APPLICATION_XML.to_string());
        let mut ser_xml_buf = Vec::new();

        let result = serde_xml_rust::to_writer(&mut ser_xml_buf, &data);

        if let Err(error) = result {
            return Err(DeboaError::SerializationError { message: error.to_string() });
        }

        self.body = Some(ser_xml_buf);

        Ok(self)
    }
}
