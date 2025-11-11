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
//! ```rust,compile_fail
//! use serde::{Serialize, Deserialize};
//! use vamo::resource::Resource;
//! use deboa::client::serde::RequestBody;
//! use deboa_extras::http::serde::json::JsonBody;
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! struct User {//!     id: Option<u64>,
//!     name: String,
//!     email: String,
//! }
//!
//! impl Resource for User {
//!     fn id(&self) -> String {
//!         self.id.map(|id| id.to_string()).unwrap_or_default()
//!     }
//!
//!     fn post_path(&self) -> &str { "/users" }
//!     fn delete_path(&self) -> &str { "/users/{}" }
//!     fn put_path(&self) -> &str { "/users/{}" }
//!     fn patch_path(&self) -> &str { "/users/{}" }
//!     
//!     fn body_type(&self) -> impl RequestBody {
//!         JsonBody
//!     }
//! }
//! ```
//!
//! ## Request Traits
//!
//! The module provides several traits for creating different types of HTTP requests:
//! - `AsPostRequest`: Create POST requests for creating resources
//! - `AsDeleteRequest`: Create DELETE requests for removing resources
//! - `AsPutRequest`: Create PUT requests for full updates
//! - `AsPatchRequest`: Create PATCH requests for partial updates

use deboa::{
    client::serde::RequestBody,
    errors::{DeboaError, RequestError},
    request::DeboaRequest,
    Result,
};
use serde::Serialize;
use std::str::FromStr;
use url::Url;

/// Trait to be implemented by resources.
pub trait Resource {
    /// Returns the id of the resource.
    ///
    /// # Returns
    ///
    /// * `String` - The id of the resource.
    ///
    fn id(&self) -> String;
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
    fn add_path(&self, path: &str) -> Result<Url> {
        let url = Url::from_str("http://deboa");
        if let Err(e) = url {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
        }
        let final_path = path.replace("{}", &self.id());
        let full_url = url.unwrap().join(&final_path);
        if let Err(e) = full_url {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
        }
        Ok(full_url.unwrap())
    }
}

/// Trait to be implemented by resources to be used as post request.s
pub trait AsPostRequest<T: Resource> {
    /// Returns the post request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The post request.
    ///
    fn as_post_request(&self) -> Result<DeboaRequest>;
}

impl<T: Resource + Serialize> AsPostRequest<T> for T {
    fn as_post_request(&self) -> Result<DeboaRequest> {
        DeboaRequest::post(self.add_path(self.post_path())?)?
            .body_as(self.body_type(), self)?
            .build()
    }
}

/// Trait to be implemented by resources to be used as delete request.s
pub trait AsDeleteRequest<T: Resource> {
    /// Returns the delete request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The delete request.
    ///
    fn as_delete_request(&self) -> Result<DeboaRequest>;
}

impl<T: Resource + Serialize> AsDeleteRequest<T> for T {
    fn as_delete_request(&self) -> Result<DeboaRequest> {
        DeboaRequest::delete(self.add_path(self.delete_path())?)?.build()
    }
}

/// Trait to be implemented by resources to be used as put request.s
pub trait AsPutRequest<T: Resource> {
    /// Returns the put request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The put request.
    ///
    fn as_put_request(&self) -> Result<DeboaRequest>;
}

impl<T: Resource + Serialize> AsPutRequest<T> for T {
    fn as_put_request(&self) -> Result<DeboaRequest> {
        DeboaRequest::put(self.add_path(self.put_path())?)?
            .body_as(self.body_type(), self)?
            .build()
    }
}

/// Trait to be implemented by resources to be used as patch request.s
pub trait AsPatchRequest<T: Resource> {
    /// Returns the patch request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The patch request.
    ///
    fn as_patch_request(&self) -> Result<DeboaRequest>;
}

impl<T: Resource + Serialize> AsPatchRequest<T> for T {
    fn as_patch_request(&self) -> Result<DeboaRequest> {
        DeboaRequest::patch(self.add_path(self.patch_path())?)?
            .body_as(self.body_type(), self)?
            .build()
    }
}
