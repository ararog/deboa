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
//!     fn load_path(&self) -> &str { "/users/:id" }
//!     fn create_path(&self) -> &str { "/users" }
//!     fn remove_path(&self) -> &str { "/users/:id" }
//!     fn update_path(&self) -> &str { "/users/:id" }
//!     fn edit_path(&self) -> &str { "/users/:id" }
//!     
//!     fn body_type(&self) -> impl RequestBody {
//!         JsonBody
//!     }
//! }
//! ```
use deboa::{client::serde::RequestBody, Result};
use serde::Serialize;

/// Trait to be implemented by structs which will behave as rest resources.
/// Resource trait will use convention over configuration to define the paths
/// for operations on resources.
/// 
/// # Type Parameters
///
/// * `R` - The resource type.
/// 
/// # Example
///
/// ```rust,no_run
/// use serde::{Serialize, Deserialize};
/// use vamo::resource::Resource;
/// use deboa::{Result, client::serde::RequestBody};
/// use deboa_extras::http::serde::json::JsonBody;
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct User {
///     id: Option<u64>,
///     name: String,
///     email: String,
/// }
///
/// impl Resource for User {
///     fn id(&self) -> String {
///         self.id.map(|id| id.to_string()).unwrap_or_default()
///     }
///
///     fn name(&self) -> &str { "users" }
///     
///     fn body_type(&self) -> impl RequestBody {
///         JsonBody
///     }
/// }
/// ```
pub trait Resource {
    /// Returns the id of resource.
    ///
    /// # Returns
    ///
    /// * `String` - The id of resource.
    ///
    fn id(&self) -> String;
    /// Returns the name of resource.
    ///
    /// # Returns
    ///
    /// * `&str` - The name of resource.
    ///
    fn name(&self) -> &str;
    /// Returns the body type of resource.
    ///
    /// # Returns
    ///
    /// * `impl RequestBody` - The body type of resource.
    ///
    fn body_type(&self) -> impl RequestBody;
    /// Adds a path to the url.
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
/// 
/// # Type Parameters
///
/// * `R` - The resource type.
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
/// let response = vamo.create(&mut post)?.send().await?;
/// ```
pub trait ResourceMethod<R>
where
    R: Resource + Serialize,
{
    /// Get a resource from REST endpoint
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The result of the get operation.
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
    /// let response = vamo.load(&mut post)?.send().await?;
    /// ```
    fn load(&mut self, resource: &mut R) -> Result<&mut Self>;
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
    /// let response = vamo.create(&mut post)?.send().await?;
    /// ```
    fn create(&mut self, resource: &mut R) -> Result<&mut Self>;
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
    /// let response = vamo.update(&mut post)?.send().await?;
    /// ```
    fn update(&mut self, resource: &mut R) -> Result<&mut Self>;
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
    /// let response = vamo.edit(&mut post)?.send().await?;
    /// ```
    fn edit(&mut self, resource: &mut R) -> Result<&mut Self>;
    /// Delete a resource to REST endpoint
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource to be deleted.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The result of the delete operation.
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
    /// let response = vamo.remove(&mut post)?.send().await?;
    /// ```
    fn remove(&mut self, resource: &mut R) -> Result<&mut Self>;
}
