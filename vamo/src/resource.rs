//! # Resource Module
//!
//! This module provides traits and implementations for working with RESTful resources
//! in the `vamo` HTTP client. It simplifies the process of creating, reading, updating,
//! and deleting resources by providing a type-safe, trait-based interface.
//!
//! ## Features
//!
//! - **Resource Trait**: Define RESTful resources with customizable endpoints
//! - **Request Builders**: Generate HTTP requests for CRUD operations
//! - **Type Safety**: Compile-time checking of request/response types
//! - **URL Handling**: Automatic path parameter substitution
//!
//! ## Usage
//!
//! The `Resource` trait should be implemented for types that represent API resources.
//! The `vamo-macros` crate provides a derive macro that can automatically implement
//! this trait for your types.
//!
//! ## Example
//!
//! ```rust,ignore
//! use serde::{Serialize, Deserialize};
//! use vamo::resource::Resource;
//! use deboa::{Result, client::serde::RequestBody};
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {
//!     id: Option<u64>,
//!     name: String,
//!     email: String,
//! }
//!
//! impl Resource for User {
//!     fn id(&self) -> String {
//!         self.id.map(|id| id.to_string()).unwrap_or_default()
//!     }
//!
//!     fn get_path(&self) -> &str { "/users/:id" }
//!     fn post_path(&self) -> &str { "/users" }
//!     fn delete_path(&self) -> &str { "/users/:id" }
//!     fn put_path(&self) -> &str { "/users/:id" }
//!     fn patch_path(&self) -> &str { "/users/:id" }
//!     
//!     fn body_type(&self) -> impl RequestBody {
//!         JsonBody
//!     }
//! }
//! ```
use deboa::{client::serde::RequestBody, Result};
use serde::Serialize;

/// Trait to be implemented by resources.
pub trait Resource {
    /// Returns the id of the resource.
    ///
    /// # Returns
    ///
    /// * `String` - The id of the resource.
    ///
    fn id(&self) -> String;
    /// Returns the get path of the resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The get path of the resource.
    ///
    fn get_path(&self) -> &str;
    /// Returns the post path of the resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The post path of the resource.
    ///
    fn post_path(&self) -> &str;
    /// Returns the delete path of the resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The delete path of the resource.
    ///
    fn delete_path(&self) -> &str;
    /// Returns the put path of the resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The put path of the resource.
    ///
    fn put_path(&self) -> &str;
    /// Returns the patch path of the resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The patch path of the resource.
    ///
    fn patch_path(&self) -> &str;
    /// Returns the body type of the resource.
    ///
    /// # Returns
    ///
    /// * `impl RequestBody` - The body type of the resource.
    ///
    fn body_type(&self) -> impl RequestBody;
    /// Adds a path to the resource.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be added.
    ///
    /// # Returns
    ///
    /// * `Result<Url>` - The url with the path added.
    ///
    fn add_path(&self, path: &str) -> String {
        path.replace(":id", &self.id())
    }
}

/// Trait which allow http methods on resources
pub trait ResourceMethod<R>
where
    R: Resource + Serialize,
{
    /// Post a resource to REST endpoint
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be posted.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The result of the post operation.
    /// 
    /// # Example
    ///
    /// ```rust,compile_fail
    /// use vamo::{Vamo, resource::{Resource, ResourceMethod}};
    /// 
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// // Assuming Post is a Resource
    /// let mut post = Post {
    ///     id: 1,
    ///     title: "Some title".to_string(),
    ///     body: "Some body".to_string(),
    ///     user_id: 1,
    /// };
    /// let response = vamo.post_resource(&mut post)?.send().await?;
    /// ```
    fn post_resource(&mut self, resource: &mut R) -> Result<&mut Self>;
    /// Put a resource to REST endpoint
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be put.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The result of the put operation.
    ///
    /// # Example
    ///
    /// ```rust,compile_fail
    /// use vamo::{Vamo, resource::{Resource, ResourceMethod}};
    /// 
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// // Assuming Post is a Resource
    /// let mut post = Post {
    ///     id: 1,
    ///     title: "Some title".to_string(),
    ///     body: "Some body".to_string(),
    ///     user_id: 1,
    /// };
    /// let response = vamo.put_resource(&mut post)?.send().await?;
    /// ```
    fn put_resource(&mut self, resource: &mut R) -> Result<&mut Self>;
    /// Patch a resource to REST endpoint
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be patched.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The result of the patch operation.
    ///
    /// # Example
    ///
    /// ```rust,compile_fail
    /// use vamo::{Vamo, resource::{Resource, ResourceMethod}};
    /// 
    /// let mut vamo = Vamo::new("https://api.example.com")?;
    /// // Assuming Post is a Resource
    /// let mut post = Post {
    ///     id: 1,
    ///     title: "Some title".to_string(),
    ///     body: "Some body".to_string(),
    ///     user_id: 1,
    /// };
    /// let response = vamo.patch_resource(&mut post)?.send().await?;
    /// ```
    fn patch_resource(&mut self, resource: &mut R) -> Result<&mut Self>;
}
