#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[cfg(any(
    all(feature = "tokio-rt", feature = "smol-rt"),
    all(feature = "tokio-rt", feature = "compio-rt"),
    all(feature = "smol-rt", feature = "compio-rt")
))]
compile_error!("Only one runtime feature can be enabled at a time.");

use base64::{engine::general_purpose::STANDARD, Engine as _};
use bytes::{Buf, Bytes};
use http::{header, HeaderName, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::Request;
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use url::{form_urlencoded, Url};

pub use crate::errors::DeboaError;
#[cfg(feature = "middlewares")]
pub use crate::middlewares::DeboaMiddleware;
pub use crate::{request::RequestMethod, response::DeboaResponse};

pub mod errors;
#[cfg(feature = "middlewares")]
pub mod middlewares;
pub mod request;
pub mod response;
mod runtimes;

#[cfg(test)]
mod tests;

#[allow(dead_code)]
pub const APPLICATION_XML: &str = "application/xml";

pub struct Deboa {
    base_url: Url,
    headers: Option<HashMap<HeaderName, String>>,
    query_params: Option<HashMap<&'static str, &'static str>>,
    body: Option<Vec<u8>>,
    retries: u32,
    connection_timeout: u64,
    request_timeout: u64,
    #[cfg(feature = "middlewares")]
    middleware: Option<Box<dyn DeboaMiddleware>>,
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
            .field("base_url", &self.base_url)
            .field("headers", &self.headers)
            .field("query_params", &self.query_params)
            .field("body", &self.body)
            .finish()
    }
}

impl Deboa {
    /// Allow create a new Deboa instance.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base url of the api.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn new(base_url: &str) -> Result<Self, DeboaError> {
        let default_headers: HashMap<HeaderName, String> = HashMap::from([
            (header::ACCEPT, mime::APPLICATION_JSON.to_string()),
            (header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string()),
        ]);

        let base_url = Url::parse(base_url);
        if let Err(e) = base_url {
            return Err(DeboaError::UrlParseError { message: e.to_string() });
        }

        Ok(Deboa {
            base_url: base_url.unwrap(),
            headers: Some(default_headers),
            query_params: None,
            body: None,
            retries: 0,
            connection_timeout: 0,
            request_timeout: 0,
            #[cfg(feature = "middlewares")]
            middleware: None,
        })
    }

    /// Allow add header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to add.
    /// * `value` - The header value to add.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use mime;
    /// use http::header;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.add_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref().to_string());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn add_header(&mut self, key: HeaderName, value: String) -> &mut Self {
        self.headers.as_mut().unwrap().insert(key, value);
        self
    }

    /// Allow remove header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to remove.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use http::header;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.remove_header(header::CONTENT_TYPE);
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn remove_header(&mut self, key: HeaderName) -> &mut Self {
        self.headers.as_mut().unwrap().remove(&key);
        self
    }

    /// Allow check if header exists at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the header exists, false otherwise.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use http::header;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.has_header(&header::CONTENT_TYPE);
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn has_header(&self, key: &HeaderName) -> bool {
        self.headers.as_ref().unwrap().contains_key(key)
    }

    /// Allow edit header at any time.
    ///
    /// # Arguments
    ///
    /// * `header` - The header to edit.
    /// * `value` - The new header value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use http::header;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.edit_header(header::CONTENT_TYPE, "application/json".to_string());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn edit_header(&mut self, header: HeaderName, value: String) -> &mut Self {
        if !self.has_header(&header) {
            self.add_header(header, value);
        } else {
            // We can safely unwrap here, as we have made sure that it exists by the previous if statement.
            let header_value = self.get_mut_header(&header).unwrap();

            *header_value = value;
        }

        self
    }

    /// Allow get mutable header at any time.
    ///
    /// # Arguments
    ///
    /// * `header` - The header to get.
    ///
    /// # Returns
    ///
    /// * `Option<&mut String>` - The header value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use http::header;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.get_mut_header(&header::CONTENT_TYPE);
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn get_mut_header(&mut self, header: &HeaderName) -> Option<&mut String> {
        self.headers.as_mut().unwrap().get_mut(header)
    }

    /// Allow add bearer auth at any time.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to be used in the Authorization header.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.add_bearer_auth("token".to_string());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn add_bearer_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Bearer {token}");
        if !self.has_header(&header::AUTHORIZATION) {
            self.add_header(header::AUTHORIZATION, auth);
        }
        self
    }

    /// Allow add basic auth at any time.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.add_basic_auth("username".to_string(), "password".to_string());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn add_basic_auth(&mut self, username: String, password: String) -> &mut Self {
        let auth = format!("Basic {}", STANDARD.encode(format!("{username}:{password}")));
        if !self.has_header(&header::AUTHORIZATION) {
            self.add_header(header::AUTHORIZATION, auth);
        }
        self
    }

    /// Allow change request base url at any time.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The new base url.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_base_url("https://jsonplaceholder.typicode.com")?.get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_base_url(&mut self, base_url: &str) -> Result<&mut Self, DeboaError> {
        let url = Url::parse(base_url);
        if let Err(e) = url {
            return Err(DeboaError::UrlParseError { message: e.to_string() });
        }

        self.base_url = url.unwrap();

        Ok(self)
    }

    /// Allow get request base url at any time.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let base_url = api.base_url();
    ///   println!("Base URL: {}", base_url);
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn base_url(&self) -> String {
        self.base_url.to_string()
    }

    /// Allow change request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_retries(3).get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_retries(&mut self, retries: u32) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Allow change request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_connection_timeout(5).get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_connection_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connection_timeout = timeout;
        self
    }

    /// Allow change request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_request_timeout(5).get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_request_timeout(&mut self, timeout: u64) -> &mut Self {
        self.request_timeout = timeout;
        self
    }

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
    /// use deboa::{Deboa, DeboaError};
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
    pub fn set_json<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
        self.edit_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string());
        let result = serde_json::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::SerializationError { message: error.to_string() });
        }

        self.body = Some(result.unwrap());

        Ok(self)
    }

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
    pub fn set_xml<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
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

    #[cfg(feature = "msgpack")]
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
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_msgpack(Post { id: 1, title: "title".to_string(), body: "body".to_string() })?.post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_msgpack<T: Serialize>(&mut self, data: T) -> Result<&mut Self, DeboaError> {
        let result = rmp_serde::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::SerializationError { message: error.to_string() });
        }

        self.body = Some(result.unwrap());

        Ok(self)
    }

    /// Allow set text body at any time.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.set_text("text".to_string()).post("/posts").await;
    ///   assert!(response.is_err());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.body = Some(text.as_bytes().to_vec());
        self
    }

    /// Allow add query params at any time.
    ///
    /// # Arguments
    ///
    /// * `params` - The query params to be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.set_query_params(Some(HashMap::from([("id", "1")])));
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_query_params(&mut self, params: Option<HashMap<&'static str, &'static str>>) -> &mut Self {
        self.query_params = params;
        self
    }

    /// Allow add middleware at any time.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to be added.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, DeboaMiddleware};
    /// use deboa::DeboaResponse;
    ///
    /// struct TestMonitor;
    ///
    /// impl DeboaMiddleware for TestMonitor {
    ///   fn on_request(&self, request: &Deboa) {
    ///     println!("Request: {:?}", request.base_url());
    ///   }
    ///
    ///   fn on_response(&self, request: &Deboa, response: &mut DeboaResponse) {
    ///     println!("Response: {:?}", response.status);
    ///   }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   api.add_middleware(TestMonitor);
    ///   Ok(())
    /// }
    ///
    pub fn add_middleware<M: DeboaMiddleware>(&mut self, middleware: M) -> &mut Self {
        self.middleware = Some(Box::new(middleware));
        self
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn get(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::GET, path).await
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
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
    ///   let response = api.set_json(Post { id: 1, title: "title".to_string(), body: "body".to_string() })?.post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn post(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::POST, path).await
    }

    /// Allow make a PUT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.put("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn put(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::PUT, path).await
    }

    /// Allow make a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.patch("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn patch(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::PATCH, path).await
    }

    /// Allow make a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.delete("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn delete(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::DELETE, path).await
    }

    /// Allow make a HEAD request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.head("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn head(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::HEAD, path).await
    }

    /// Allow make a OPTIONS request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.options("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn options(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::OPTIONS, path).await
    }

    /// Allow make a TRACE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.trace("/posts").await;
    ///   assert!(response.is_err());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn trace(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::TRACE, path).await
    }

    /// Allow make a CONNECT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.connect("/posts").await;
    ///   assert!(response.is_err());
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn connect(&self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(RequestMethod::CONNECT, path).await
    }

    /// Allow make a ANY request.
    ///
    /// # Arguments
    ///
    /// * `method` - The method to be requested.
    /// * `path` - The path to be requested.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, DeboaError, RequestMethod};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   let response = api.any(RequestMethod::GET, "/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn any(&self, method: RequestMethod, path: &str) -> Result<DeboaResponse, DeboaError> {
        let url = self.base_url.join(path);

        if let Err(e) = url {
            return Err(DeboaError::UrlParseError { message: e.to_string() });
        }

        let mut url = url.unwrap();

        if self.query_params.is_some() && method == RequestMethod::GET {
            let query = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(self.query_params.as_ref().unwrap())
                .finish();
            url.set_query(Some(&query));
        }

        #[cfg(feature = "middlewares")]
        if let Some(middleware) = &self.middleware {
            middleware.on_request(self);
        }

        #[cfg(feature = "tokio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::tokio::get_connection(&url).await?;

            tokio::spawn(async move {
                match conn.await {
                    Ok(_) => (),
                    Err(_err) => {
                        // return Err(DeboaError::ConnectionError {
                        //     host: url.to_string(),
                        //     message: err.to_string(),
                        // });
                    }
                };
            });

            sender
        };

        #[cfg(feature = "smol-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::smol::get_connection(&url).await.map_err(|err| DeboaError::ConnectionError {
                host: url.to_string(),
                message: err.to_string(),
            })?;

            match conn.await {
                Ok(_) => (),
                Err(err) => {
                    return Err(DeboaError::ConnectionError {
                        host: url.to_string(),
                        message: err.to_string(),
                    });
                }
            };

            sender
        };

        #[cfg(feature = "compio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::compio::get_connection(&url).await.map_err(|err| DeboaError::ConnectionError {
                host: url.to_string(),
                message: err.to_string(),
            })?;

            match conn.await {
                Ok(_) => (),
                Err(err) => {
                    return Err(DeboaError::ConnectionError {
                        host: url.to_string(),
                        message: err.to_string(),
                    });
                }
            };

            sender
        };

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if let Some(headers) = &self.headers {
                headers.iter().fold(req_headers, |acc, (key, value)| {
                    acc.insert(key, HeaderValue::from_str(value).unwrap());
                    acc
                });
            }
        }

        let req = match &self.body {
            Some(body) => builder.body(Full::new(Bytes::from_owner(body.clone()))),
            None => builder.body(Full::new(Bytes::from_owner(Vec::new()))),
        };

        if let Err(err) = req {
            return Err(DeboaError::RequestError {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let request = req.unwrap();

        // We need sure that we do not reconstruct the request somewhere else in the code as it will lead to the headers deletion making a request invalid.
        let res = sender.send_request(request).await;

        if let Err(err) = res {
            return Err(DeboaError::RequestError {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let res = res.unwrap();

        let status_code = res.status();
        let headers = res.headers().clone();

        let result = res.collect().await;

        if let Err(err) = result {
            return Err(DeboaError::DeserializationError { message: err.to_string() });
        }

        let mut response_body = result.unwrap().aggregate();

        let raw_body = response_body.copy_to_bytes(response_body.remaining()).to_vec();

        if !status_code.is_success() {
            return Err(DeboaError::RequestError {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: format!("Request failed with status code: {status_code}"),
            });
        }

        #[cfg(feature = "middlewares")]
        let mut response = DeboaResponse {
            status: status_code,
            headers,
            raw_body,
        };

        #[cfg(not(feature = "middlewares"))]
        let response = DeboaResponse {
            status: status_code,
            headers,
            raw_body,
        };

        #[cfg(feature = "middlewares")]
        if let Some(middleware) = self.middleware.as_ref() {
            middleware.on_response(self, &mut response)
        }

        Ok(response)
    }
}
