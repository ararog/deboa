use deboa::{errors::DeboaError, request::DeboaRequest, client::serde::RequestBody};
use serde::Serialize;
use std::str::FromStr;
use url::Url;

pub trait Resource {
    fn post_path(&self) -> &str;
    fn put_path(&self) -> &str;
    fn patch_path(&self) -> &str;
    fn body_type(&self) -> impl RequestBody;
    fn add_path(&self, path: &str) -> Result<Url, DeboaError> {
      let url = Url::from_str("http://deboa");
      if let Err(e) = url {
          return Err(DeboaError::UrlParse { message: e.to_string() });
      }
      let full_url = url.unwrap().join(path);
      if let Err(e) = full_url {
          return Err(DeboaError::UrlParse { message: e.to_string() });
      }
      Ok(full_url.unwrap())
  }  
}

pub trait AsPostRequest<T: Resource> {
    fn as_post_request(&self) -> Result<DeboaRequest, DeboaError>;
}


impl<T: Resource + Serialize> AsPostRequest<T> for T {
    fn as_post_request(&self) -> Result<DeboaRequest, DeboaError> {
        DeboaRequest::post(self.add_path(self.post_path())?)?.body_as(self.body_type(), self)?.build()
    }
}

pub trait AsPutRequest<T: Resource> {
    fn as_put_request(&self) -> Result<DeboaRequest, DeboaError>;
}

impl<T: Resource + Serialize> AsPutRequest<T> for T {
    fn as_put_request(&self) -> Result<DeboaRequest, DeboaError> {
        DeboaRequest::put(self.add_path(self.put_path())?)?.body_as(self.body_type(), self)?.build()
    }
}

pub trait AsPatchRequest<T: Resource> {
    fn as_patch_request(&self) -> Result<DeboaRequest, DeboaError>;
}

impl<T: Resource + Serialize> AsPatchRequest<T> for T {
    fn as_patch_request(&self) -> Result<DeboaRequest, DeboaError> {
        DeboaRequest::patch(self.add_path(self.patch_path())?)?.body_as(self.body_type(), self)?.build()
    }
}
