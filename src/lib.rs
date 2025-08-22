#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[cfg(any(
    all(feature = "tokio-rt", feature = "smol-rt"),
    all(feature = "tokio-rt", feature = "compio-rt"),
    all(feature = "smol-rt", feature = "compio-rt")
))]
compile_error!("Only one runtime feature can be enabled at a time.");

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use http::{header, HeaderName, HeaderValue};
use http_body_util::BodyExt;
use hyper::Request;
#[cfg(any(feature = "json", feature = "xml"))]
use serde::Serialize;
use std::collections::HashMap;
use url::{form_urlencoded, Url};

#[cfg(feature = "middlewares")]
pub use crate::middlewares::{DeboaMiddleware};
pub use crate::{request::RequestMethod, response::DeboaResponse};

#[cfg(feature = "middlewares")]
pub mod middlewares;
pub mod request;
pub mod response;
mod runtimes;
mod tests;

pub struct Deboa {
    base_url: &'static str,
    headers: Option<HashMap<HeaderName, &'static str>>,
    params: Option<HashMap<&'static str, &'static str>>,
    body: Option<String>,
    #[cfg(feature = "middlewares")]
    middleware: Option<Box<dyn DeboaMiddleware>>,
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
    /// let api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// ```
    ///
    pub fn new(base_url: &'static str) -> Self {
        let default_headers: HashMap<HeaderName, &'static str> = HashMap::from([
            (header::ACCEPT, mime::APPLICATION_JSON.as_ref()),
            (header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref()),
        ]);

        Deboa {
            base_url,
            headers: Some(default_headers),
            params: None,
            body: None,
            #[cfg(feature = "middlewares")]
            middleware: None,
        }
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
    /// let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// api.add_header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref());
    /// ```
    ///
    pub fn add_header(&mut self, key: HeaderName, value: String) -> &mut Self {
        self.headers.as_mut().unwrap().insert(key, value.leak());
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
    /// let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// api.remove_header(header::CONTENT_TYPE);
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
    /// let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// api.has_header(header::CONTENT_TYPE);
    /// ```
    ///
    pub fn has_header(&self, key: HeaderName) -> bool {
        self.headers.as_ref().unwrap().contains_key(&key)
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
    /// let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// api.add_bearer_auth("token");
    /// ```
    ///
    pub fn add_bearer_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Bearer {token}");
        if !self.has_header(header::AUTHORIZATION) {
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
    /// let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    /// api.add_basic_auth("username", "password");
    /// ```
    ///
    pub fn add_basic_auth(&mut self, username: String, password: String) -> &mut Self {
        let auth = format!(
            "Basic {}",
            STANDARD.encode(format!("{username}:{password}"))
        );
        if !self.has_header(header::AUTHORIZATION) {
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.set_base_url("https://jsonplaceholder.typicode.com").get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_base_url(&mut self, base_url: &'static str) -> &mut Self {
        self.base_url = base_url;
        self
    }

    /// Allow get request base url at any time.
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
    ///   let base_url = api.base_url();
    ///   println!("Base URL: {}", base_url); 
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn base_url(&self) -> &'static str {
        self.base_url
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
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
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.set_json(Post { id: 1, title: "title".to_string(), body: "body".to_string() }).post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_json<T: Serialize>(&mut self, data: T) -> &mut Self {
        match serde_json::to_string(&data) {
            Ok(json) => self.body = Some(json),
            Err(_) => self.body = None,
        }
        self
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
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
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.set_xml(Post { id: 1, title: "title".to_string(), body: "body".to_string() }).post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_xml<T: Serialize>(&mut self, data: T) -> &mut Self {
        match serde_xml_rs::to_string(&data) {
            Ok(xml) => self.body = Some(xml),
            Err(_) => self.body = None,
        }
        self
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.set_text("text".to_string()).post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.body = Some(text);
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
    /// use std::collections::HashMap;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.set_query_params(Some(HashMap::from([("id", "1")])));
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn set_query_params(&mut self, params: Option<HashMap<&'static str, &'static str>>) -> &mut Self {
        self.params = params;
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
    /// use anyhow::Result;
    /// use deboa::Deboa;
    /// use deboa::DeboaMiddleware;
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
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   api.add_middleware(Box::new(TestMonitor));
    ///   Ok(())
    /// }
    ///
    pub fn add_middleware(&mut self, middleware: Box<dyn DeboaMiddleware>) -> &mut Self {
        self.middleware = Some(middleware);
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.get("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn get(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.post("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn post(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.put("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn put(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.patch("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn patch(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.delete("/posts/1").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn delete(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.head("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn head(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.options("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn options(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.trace("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn trace(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.connect("/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn connect(&self, path: &str) -> Result<DeboaResponse> {
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
    /// use anyhow::Result;
    /// use deboa::{Deboa, RequestMethod};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut api = Deboa::new("https://jsonplaceholder.typicode.com");
    ///   let response = api.any(RequestMethod::GET, "/posts").await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub async fn any(&self, method: RequestMethod, path: &str) -> Result<DeboaResponse> {
        let mut url = Url::parse(format!("{}{}", self.base_url, path).as_str()).unwrap();

        if self.params.is_some() && method == RequestMethod::GET {
            let query = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(self.params.as_ref().unwrap())
                .finish();
            url.set_query(Some(&query));
        }

        #[cfg(feature = "middlewares")]
        if let Some(middleware) = self.middleware.as_ref() {
            middleware.on_request(self);
        }

        #[cfg(feature = "tokio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::tokio::get_connection(&url).await?;

            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            });

            sender
        };

        #[cfg(feature = "smol-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::smol::get_connection(&url).await?;

            smol::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            })
            .detach();

            sender
        };

        #[cfg(feature = "compio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::compio::get_connection(&url).await?;

            compio::runtime::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            })
            .detach();

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
                    acc.insert(key, HeaderValue::from_static(value));
                    acc
                });
            }
        }

        let req = match &self.body {
            Some(body) => builder.body(body.clone())?,
            None => builder.body(String::new())?,
        };

        let res = sender.send_request(req).await?;

        #[cfg(feature = "middlewares")]
        let mut response = DeboaResponse {
            status: res.status(),
            headers: res.headers().clone(),
            body: Box::new(res.collect().await?.aggregate()),
        };

        #[cfg(not(feature = "middlewares"))]
        let response = DeboaResponse {
            status: res.status(),
            headers: res.headers().clone(),
            body: Box::new(res.collect().await?.aggregate()),
        };

        #[cfg(feature = "middlewares")]
        if let Some(middleware) = self.middleware.as_ref() {
            middleware.on_response(self, &mut response)
        }

        Ok(response)
    }
}