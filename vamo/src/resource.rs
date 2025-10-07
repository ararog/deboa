use deboa::{client::serde::RequestBody, errors::DeboaError, request::DeboaRequest, Result};
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
            return Err(DeboaError::UrlParse {
                message: e.to_string(),
            });
        }
        let final_path = path.replace("{}", &self.id());
        let full_url = url.unwrap().join(&final_path);
        if let Err(e) = full_url {
            return Err(DeboaError::UrlParse {
                message: e.to_string(),
            });
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
