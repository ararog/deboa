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
//!     .send_with(&mut vamo)
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

use std::sync::Arc;

use crate::resource::{Resource, ResourceMethod};
use deboa::{
    client::serde::RequestBody, request::DeboaRequest, response::DeboaResponse, url::IntoUrl,
    Deboa, Result,
};
use http::{
    header::{CONTENT_TYPE, HOST},
    HeaderMap, HeaderName, HeaderValue, Method,
};
use serde::Serialize;
use url::Url;

pub mod resource;

#[cfg(test)]
mod tests;

pub struct Vamo {
    client: Deboa,
    base_url: Url,
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Arc<[u8]>,
}

impl Vamo {
    pub fn new<U: IntoUrl>(url: U) -> Result<Vamo> {
        let base_url = url.into_url()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            HOST,
            HeaderValue::from_str(
                base_url
                    .host_str()
                    .unwrap(),
            )
            .unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());

        Ok(Vamo {
            client: Deboa::new(),
            base_url,
            path: String::new(),
            method: Method::GET,
            headers,
            body: Arc::new([]),
        })
    }

    pub fn client(&mut self, client: Deboa) -> &mut Self {
        self.client = client;
        self
    }

    pub fn header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

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

    pub fn get(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::GET;
        self
    }

    pub fn post(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::POST;
        self
    }

    pub fn put(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PUT;
        self
    }

    pub fn delete(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::DELETE;
        self
    }

    pub fn patch(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PATCH;
        self
    }

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
    fn get_resource(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = resource.add_path(resource.get_path());
        self.method = Method::GET;
        Ok(self)
    }

    fn post_resource(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = resource
            .post_path()
            .to_string();
        self.method = Method::POST;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn put_resource(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = resource.add_path(resource.put_path());
        self.method = Method::PUT;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn patch_resource(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = resource.add_path(resource.patch_path());
        self.method = Method::PATCH;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn delete_resource(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = resource.add_path(resource.delete_path());
        self.method = Method::DELETE;
        Ok(self)
    }
}
