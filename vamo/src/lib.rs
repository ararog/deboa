//! # Vamo: A High-Level HTTP Client for Deboa
//!
//! `vamo` provides an ergonomic, high-level API on top of the `deboa` HTTP client,
//! making it easier to work with RESTful APIs and other HTTP services. It offers
//! a more intuitive interface for building and sending HTTP requests while maintaining
//! full compatibility with the underlying `deboa` client.
//!
//! ## Features
//!
//! - **Fluent API**: Chainable methods for building and sending requests
//! - **Resource-Oriented**: First-class support for REST resources with the `Resource` trait
//! - **Authentication**: Built-in support for common authentication methods
//! - **Type Safety**: Strong typing for request/response bodies
//! - **Flexible**: Works with any HTTP method and content type
//! - **Async by Default**: Built on top of async/await for high performance
//!
//! ## Getting Started
//!
//! Add `vamo` and its dependencies to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vamo = { version = "0.1", path = "../vamo" }
//! deboa = { version = "0.1.0", path = ".." }
//! deboa-extras = { version = "0.1", path = "../deboa-extras" }
//! serde = { version = "1.0", features = ["derive"] }
//! tokio = { version = "1.0", features = ["full"] }
//! ```
//!
//! ## Basic Usage
//!
//! ### Making Simple Requests
//!
//! ```ignore
//! use vamo::Vamo;
//! use deboa::Result;
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create a new Vamo client with a base URL
//!     let mut vamo = Vamo::new("https://api.example.com")?;
//!
//!     // Make a GET request
//!     let response = vamo
//!         .get("/users/1")?
//!         .send()
//!         .await?;
//!     
//!     // Parse response as JSON
//!     let user: User = response
//!         .body_as(JsonBody)
//!         .await?;
//!     println!("User: {:?}", user);
//!
//!     // Make a POST request with JSON body
//!     let new_user = json!({
//!         "name": "John Doe",
//!         "email": "john@example.com"
//!     });
//!     
//!     let response = vamo
//!         .post("/users")?
//!         .body_as(JsonBody, &new_user)?
//!         .send()
//!         .await?;
//!     
//!     println!("Created user: {:?}", response.status());
//!     Ok(())
//! }
//! ```
//!
//! ## Working with Resources
//!
//! Vamo provides a `Resource` trait that makes it easy to work with REST resources:
//!
//! ```ignore
//! use deboa::Result;
//! use deboa_extras::http::serde::json::JsonBody;
//! use serde::{Deserialize, Serialize};
//! use vamo::{Vamo, resource::{Resource, ResourceMethod}};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     id: Option<u64>,
//!     name: String,
//!     email: String,
//! }
//!
//! impl Resource for User {
//!     // Return the resource ID as a string
//!     fn id(&self) -> String {
//!         self.id.map(|id| id.to_string()).unwrap_or_default()
//!     }
//!     
//!     // Return the base path for this resource (e.g., "users")
//!     fn name(&self) -> &str {
//!         "users"
//!     }
//!     
//!     // Specify how to serialize this resource
//!     fn body_type(&self) -> impl deboa::client::serde::RequestBody {
//!         JsonBody
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut vamo = Vamo::new("https://api.example.com")?;
//!     
//!     // List all users
//!     let mut user_template = User {
//!         id: None,
//!         name: String::new(),
//!         email: String::new(),
//!     };
//!     
//!     let users: Vec<User> = vamo
//!        .load(&mut user_template)?
//!        .send()
//!        .await?
//!        .body_as(JsonBody)
//!        .await?;
//!     println!("All users: {:?}", users);
//!     
//!     // Create a new user
//!     let mut new_user = User {
//!         id: None,
//!         name: "John Doe".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let created: User = vamo
//!        .create(&mut new_user)?
//!        .send()
//!        .await?
//!        .body_as(JsonBody)
//!        .await?;
//!     println!("Created user: {:?}", created);
//!     
//!     // Update a user
//!     let mut updated_user = User {
//!         id: created.id,
//!         name: "John Updated".to_string(),
//!         email: created.email,
//!     };
//!     
//!     let updated: User = vamo
//!        .update(&mut updated_user)?
//!        .send()
//!        .await?
//!        .body_as(JsonBody)
//!        .await?;
//!     println!("Updated user: {:?}", updated);
//!     
//!     // Delete a user
//!     vamo
//!       .remove(&mut updated_user)?
//!       .send()
//!       .await?;
//!     println!("User deleted");
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Authentication
//!
//! Vamo provides convenience methods for common authentication methods:
//!
//! ```ignore
//! use vamo::Vamo;
//! use deboa::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Bearer token authentication
//!     let mut vamo = Vamo::new("https://api.example.com")?;
//!     vamo
//!       .get("/users/1")
//!       .bearer_auth("your-token-here")
//!       .send()
//!       .await?;
//!
//!     // Basic authentication
//!     let mut vamo = Vamo::new("https://api.example.com")?;
//!     vamo
//!       .get("/users/1")
//!       .basic_auth("username", "password")
//!       .send()
//!       .await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Error Handling
//!
//! Vamo uses the `deboa::Result` type for error handling, which provides detailed
//! error information including:
//! - Network errors
//! - Serialization/deserialization errors
//! - HTTP protocol errors
//! - URL parsing errors
//!
//! ## Examples
//!
//! Check the `examples/` directory for more comprehensive examples of using Vamo
//! with different types of APIs and authentication methods.
//!
//! ## License
//!
//! MIT license
//!
//! ## Author
//!
//! Rogerio Pacheco <rogerio.pacheco@gmail.com>
use std::sync::Arc;

use crate::resource::{Resource, ResourceMethod};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use deboa::{
    client::serde::RequestBody,
    errors::{DeboaError, RequestError},
    request::DeboaRequest,
    response::DeboaResponse,
    url::IntoUrl,
    Client, Result,
};
use http::{
    header::{self, CONTENT_TYPE, HOST},
    HeaderMap, HeaderName, HeaderValue, Method,
};
use serde::Serialize;
use url::Url;

pub mod resource;

#[cfg(test)]
mod tests;

/// A builder for HTTP requests.
pub struct Vamo {
    client: Client,
    base_url: Url,
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Arc<[u8]>,
}

impl Vamo {
    /// Create a new Vamo instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the requests.
    ///
    /// # Returns
    ///
    /// * `Result<Vamo>` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    ///
    /// # Panics
    ///
    /// If the URL is invalid, or headers are invalid, the function will panic.
    ///
    pub fn new<U: IntoUrl>(url: U) -> Result<Vamo> {
        let base_url = url.into_url()?;
        let mut headers = HeaderMap::new();
        let host = base_url.host_str();
        if host.is_none() {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Invalid URL: Missing host.".to_string(),
            }));
        }

        let host_header = HeaderValue::from_str(
            base_url
                .host_str()
                .unwrap(),
        );
        if let Err(e) = host_header {
            return Err(DeboaError::Header { message: e.to_string() });
        }

        headers.insert(HOST, host_header.unwrap());

        let content_type_header = HeaderValue::from_str("application/json");
        if let Err(e) = content_type_header {
            return Err(DeboaError::Header { message: e.to_string() });
        }

        headers.insert(CONTENT_TYPE, content_type_header.unwrap());

        Ok(Vamo {
            client: Client::default(),
            base_url,
            path: String::new(),
            method: Method::GET,
            headers,
            body: Arc::new([]),
        })
    }

    /// Set the client to be used for requests.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used for requests.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    #[inline]
    pub fn client(&mut self, client: Client) -> &mut Self {
        self.client = client;
        self
    }

    /// Set a header for the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .header("Content-Type", "application/json")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    /// Set the body of the request.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The type of the body.
    /// * `body` - The body to be set.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The builder.
    #[inline]
    pub fn body_as<T: RequestBody, B: Serialize>(
        &mut self,
        body_type: T,
        body: B,
    ) -> Result<&mut Self> {
        self.body = body_type
            .serialize(body)?
            .into();
        Ok(self)
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    #[inline]
    pub fn get(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::GET;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.post("/path").body_as(JSON, body).send().await?;
    /// ```
    #[inline]
    pub fn post(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::POST;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.put("/path/1").body_as(JSON, body).send().await?;
    /// ```
    #[inline]
    pub fn put(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PUT;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.patch("/path/1").body_as(JsonBody, body).send().await?;
    /// ```
    #[inline]
    pub fn patch(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PATCH;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.delete("/path/1").send().await?;
    /// ```
    #[inline]
    pub fn delete(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::DELETE;
        self
    }

    /// Set the bearer token for the request.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .bearer_auth("your-token-here")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn bearer_auth(&mut self, token: &str) -> &mut Self {
        self.header(header::AUTHORIZATION, format!("Bearer {token}").as_str());
        self
    }

    /// Set the basic authentication for the request.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .basic_auth("username", "password")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            header::AUTHORIZATION,
            format!("Basic {}", STANDARD.encode(format!("{username}:{password}"))).as_str(),
        );
        self
    }

    /// Send the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Errors
    ///
    /// * `DeboaError` - The error.
    ///
    /// # Examples
    ///
    /// ``` rust, compile_fail
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    ///
    /// # Notes
    ///
    /// * The request is sent using the `Deboa` client.
    /// * The response is returned as a `DeboaResponse`.
    ///
    #[inline]
    pub async fn send(&mut self) -> Result<DeboaResponse> {
        let mut base_url = self
            .base_url
            .clone();
        let path_and_query = self
            .path
            .split_once('?');
        let path = if let Some((path, query)) = path_and_query {
            base_url.set_query(Some(query));
            path
        } else {
            &self.path
        };

        let base_path = self.base_url.path();
        if base_path == "/" {
            base_url.set_path(path);
        } else {
            base_url.set_path(&format!("{}{}", base_path, path));
        }

        let request = DeboaRequest::from(base_url.as_str())?
            .method(self.method.clone())
            .headers(self.headers.clone())
            .raw_body(&self.body)
            .build()?;

        self.client
            .execute(request)
            .await
    }
}

impl<R: Resource + Serialize> ResourceMethod<R> for Vamo {
    fn load(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::GET;
        Ok(self)
    }

    fn create(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}", resource.name());
        self.method = Method::POST;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn update(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::PUT;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn edit(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::PATCH;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn remove(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::DELETE;
        Ok(self)
    }
}
