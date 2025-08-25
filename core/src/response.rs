use http::{HeaderMap, StatusCode};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Deserialize;

use crate::DeboaError;

#[derive(PartialEq, Debug)]
pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
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
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
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
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
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
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let mut response = api.get("/posts").await?;
    ///   let posts = response.json::<Vec<Post>>().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let body = self.body.as_mut();
        let json = serde_json::from_reader(body.as_bytes());
        if let Err(err) = json {
            return Err(DeboaError::DeserializationError {
                message: err.to_string(),
            });
        }

        Ok(json.unwrap())
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
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let mut response = api.get("/posts").await?;
    ///   let posts = response.xml::<Vec<Post>>().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    #[cfg(feature = "xml")]
    pub async fn xml<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T, DeboaError> {
        let body = self.body.as_mut();
        let json = serde_xml_rs::from_reader(body.as_bytes());
        if let Err(err) = json {
            return Err(DeboaError::DeserializationError {
                message: err.to_string(),
            });
        }

        Ok(json.unwrap())
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
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let mut response = api.get("/posts").await?;
    ///   let text = response.text().await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn text(&mut self) -> Result<String, DeboaError> {
        Ok(self.body.clone())
    }
}
