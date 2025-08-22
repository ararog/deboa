use anyhow::Result;
use bytes::Buf;
use http::{HeaderMap, StatusCode};
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Deserialize;

pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Box<dyn Buf>,
}

impl DeboaResponse {


  /// This method is called after the response is received.
  /// 
  /// # Examples
  ///
  /// ```rust
  /// use anyhow::Result;
  /// use deboa::Deboa;
  /// 
  /// #[tokio::main]
  /// async fn main() -> Result<()> {
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
  /// use anyhow::Result;
  /// use deboa::Deboa;
  /// 
  /// #[tokio::main]
  /// async fn main() -> Result<()> {
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
  /// use anyhow::Result;
  /// use deboa::{Deboa, RequestMethod};
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
  /// async fn main() -> Result<()> {
  ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
  ///   let mut response = api.get("/posts").await?;
  ///   let posts = response.json::<Vec<Post>>().await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
      let body = self.body.as_mut();
      let json = serde_json::from_reader(body.reader());
      if let Err(err) = json {
          return Err(err.into());
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
  /// use anyhow::Result;
  /// use deboa::{Deboa, RequestMethod};
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
  /// async fn main() -> Result<()> {
  ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
  ///   let mut response = api.get("/posts").await?;
  ///   let posts = response.xml::<Vec<Post>>().await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  #[cfg(feature = "xml")]
  pub async fn xml<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
      let body = self.body.as_mut();
      let json = serde_xml_rs::from_reader(body.reader());
      if let Err(err) = json {
          return Err(err.into());
      }

      Ok(json.unwrap())
  }

  /// This method is called after the response is received.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use anyhow::Result;
  /// use deboa::{Deboa, RequestMethod};
  /// 
  /// #[tokio::main]
  /// async fn main() -> Result<()> {
  ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
  ///   let mut response = api.get("/posts").await?;
  ///   let text = response.text().await?;
  ///   Ok(())
  /// }
  /// ```
  ///
  pub async fn text(&mut self) -> Result<String> {
    let body = self.body.as_mut();
    let text = body.copy_to_bytes(body.remaining()).to_vec();
    Ok(String::from_utf8(text)?)
  }
}
