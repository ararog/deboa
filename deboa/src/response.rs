use http::{HeaderMap, StatusCode};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Deserialize;

use crate::DeboaError;

#[derive(PartialEq, Debug)]
pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub raw_body: Vec<u8>,
}

impl DeboaResponse {
    /// This method is called after the response is received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());
    ///   let mut response = api.get("/posts").await?;
    ///   let status = response.status();
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// This method is called after the response is received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());
    ///   let mut response = api.get("/posts").await?;
    ///   let headers = response.headers();
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn headers(&self) -> HeaderMap {
        self.headers.clone()
    }

    #[cfg(feature = "json")]
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `T` - The type to be deserialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Post {
    ///     id: u32,
    ///     title: String,
    ///     body: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());
    ///   let mut response = api.get("/posts").await?;
    ///   let posts = response.json::<Vec<Post>>().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let body = self.raw_body.as_ref();

        let json = serde_json::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::DeserializationError { message: err.to_string() }),
        }
    }

    #[cfg(feature = "xml")]
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `T` - The type to be deserialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    /// use serde::{Serialize, Deserialize};
    /// use http::header;
    ///
    /// #[derive(Serialize, Deserialize)]
    /// #[serde(rename_all = "PascalCase")]
    /// struct Response {
    ///     response_code: u32,
    ///     response_message: String,
    /// }
    ///
    /// #[tokio::main]
    /// /*
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://reqbin.com".to_string());
    ///   api.edit_header(header::CONTENT_TYPE, deboa::APPLICATION_XML.to_string());
    ///   api.edit_header(header::ACCEPT, deboa::APPLICATION_XML.to_string());
    ///
    ///   let mut response = api.get("/echo/get/xml").await?;
    ///   let posts = response.xml::<Response>().await?;
    ///   Ok(())
    /// }
    /// */
    ///
    #[cfg(feature = "xml")]
    pub async fn xml<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let body: &[u8] = self.raw_body.as_ref();
        let xml = serde_xml_rust::from_reader(body);

        match xml {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::DeserializationError { message: err.to_string() }),
        }
    }

    #[cfg(feature = "msgpack")]
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `T` - The type to be deserialized.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct Post {
    ///     id: u32,
    ///     title: String,
    ///     body: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());
    ///   let mut response = api.get("/posts").await?;
    ///   let posts = response.msgpack::<Vec<Post>>().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    #[cfg(feature = "msgpack")]
    pub async fn msgpack<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let body = self.raw_body.as_ref();

        let rmp_deserialized = rmp_serde::from_slice::<T>(&body);

        match rmp_deserialized {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::DeserializationError { message: err.to_string() }),
        }
    }

    /// This method is called after the response is received.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());
    ///   let mut response = api.get("/posts").await?;
    ///   let text = response.text().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn text(&mut self) -> Result<String, DeboaError> {
        String::from_utf8(self.raw_body.clone()).map_err(|err| DeboaError::SerializationError { message: err.to_string() })
    }
}
