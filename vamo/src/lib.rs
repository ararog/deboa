//! # Vamo: A High-Level HTTP Client for Deboa
//!
//! `vamo` provides a more ergonomic, high-level API on top of the `deboa` HTTP client,
//! making it easier to work with RESTful APIs and other HTTP services.
//!
//! ## Features
//!
//! - **Simplified API**: Chainable methods for building and sending requests
//! - **Base URL Management**: Automatically handles URL construction
//! - **Resource-Oriented**: Work with API resources in a more natural way
//! - **Seamless Integration**: Fully compatible with `deboa` and `deboa-extras`
//!
//! ## Getting Started
//!
//! Add `vamo` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vamo = { version = "0.1", path = "../vamo" }
//! deboa = { version = "0.1", path = ".." }
//! deboa-extras = { version = "0.1", path = "../deboa-extras" }
//! ```
//!
//! ## Basic Usage
//!
//! ```compile_fail
//! use vamo::Vamo;
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   // Create a new Vamo client with a base URL
//!   let mut vamo = Vamo::new("https://api.example.com")?;
//!
//!   // Make a GET request
//!   let user: serde_json::Value = vamo
//!     .get("/users/1")?
//!     .go(&mut vamo)
//!     .await?
//!     .body_as(JsonBody)
//!     .await?;
//!
//!   println!("User: {:?}", user);
//!   Ok(())
//! }
//! ```
//!
//! ## Working with Resources
//!
//! ```compile_fail
//! use vamo::{Vamo, resource::Resource};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     id: Option<u64>,
//!     name: String,
//!     email: String,
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut vamo = Vamo::new("https://api.example.com")?;
//!   let users = Resource::new("/users", &mut vamo);
//!
//!   // List all users
//!   let all_users: Vec<User> = users.list().await?;
//!
//!   // Create a new user
//!   let new_user = User {
//!     id: None,
//!     name: "John Doe".to_string(),
//!     email: "john@example.com".to_string(),
//!   };
//!   let created: User = users.create(&new_user).await?;
//!   Ok(())
//! }
//! ```

use deboa::{
    errors::{DeboaError, RequestError},
    request::{DeboaRequest, DeboaRequestBuilder},
    response::DeboaResponse,
    url::IntoUrl,
    Deboa, Result,
};
use url::Url;

pub mod resource;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Vamo {
    client: Deboa,
    base_url: Url,
}

impl From<Vamo> for Deboa {
    fn from(val: Vamo) -> Self {
        val.client
    }
}

impl AsMut<Deboa> for Vamo {
    fn as_mut(&mut self) -> &mut Deboa {
        &mut self.client
    }
}

impl Vamo {
    /// Creates a new Vamo instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL.
    ///
    /// # Returns
    ///
    /// A Result containing the new Vamo instance.
    ///
    pub fn new<T: IntoUrl>(url: T) -> Result<Self> {
        Ok(Self {
            client: Deboa::new(),
            base_url: url.into_url()?,
        })
    }

    /// Returns a mutable reference to the Deboa client.
    ///
    /// # Returns
    ///
    /// A mutable reference to the Deboa client.
    ///
    pub fn client(&mut self) -> &mut Deboa {
        &mut self.client
    }

    /// Returns a Result containing the URL with the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing the URL with the given path.
    ///
    fn url(&self, path: &str) -> Result<Url> {
        let url = self.base_url.join(path);
        if let Err(e) = url {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
        }
        Ok(url.unwrap())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a GET request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    /// use deboa_extras::http::serde::json::JsonBody;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.get("/users/1")?.build()?;
    /// let response = request
    ///   .go(vamo)
    ///   .await?
    ///   .body_as(JsonBody)
    ///   .await?;
    /// ```
    pub fn get(&self, path: &str) -> Result<DeboaRequestBuilder> {
        DeboaRequest::get(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a POST request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    /// use deboa_extras::http::serde::json::JsonBody;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.post("/users")?.body_as(JsonBody).build()?;
    /// let response = request
    ///   .go(vamo)
    ///   .await?;
    /// ```
    pub fn post(&self, path: &str) -> Result<DeboaRequestBuilder> {
        DeboaRequest::post(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a PUT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    /// use deboa_extras::http::serde::json::JsonBody;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.put("/users/1")?.body_as(JsonBody).build()?;
    /// let response = request
    ///   .go(vamo)
    ///   .await?;
    /// ```
    pub fn put(&self, path: &str) -> Result<DeboaRequestBuilder> {
        DeboaRequest::put(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    /// use deboa_extras::http::serde::json::JsonBody;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.patch("/users/1")?.body_as(JsonBody).build()?;
    /// let response = request
    ///   .go(vamo)
    ///   .await?;
    /// ```
    pub fn patch(&self, path: &str) -> Result<DeboaRequestBuilder> {
        DeboaRequest::patch(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.delete("/users/1")?.build()?;
    /// let response = request.go(vamo).await?;
    /// ```
    pub fn delete(&self, path: &str) -> Result<DeboaRequestBuilder> {
        DeboaRequest::delete(self.url(path)?.as_str())
    }

    /// Executes a DeboaRequest.
    ///
    /// # Arguments
    ///
    /// * `request` - The DeboaRequest to execute.
    ///
    /// # Returns
    ///
    /// A Result containing the DeboaResponse.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use vamo::Vamo;
    ///
    /// let vamo = Vamo::new("https://api.example.com")?;
    /// let request = vamo.get("/users")?.build()?;
    /// let response = request
    ///   .go(vamo)
    ///   .await?
    ///   .text()
    ///   .await?;
    /// ```
    pub async fn go(&mut self, mut request: DeboaRequest) -> Result<DeboaResponse> {
        let mut url = self.base_url.to_string();
        url.push_str(request.url().path());
        let result = request.set_url(url);
        if let Err(e) = result {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
        }

        self.client.execute(request).await
    }
}
