use deboa::{
    errors::DeboaError, request::{DeboaRequest, DeboaRequestBuilder, IntoUrl}, Deboa
};
use url::Url;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Vamo {
    client: Deboa,
    base_url: Url,
}

impl Vamo {
    pub fn new<T: IntoUrl>(url: T) -> Result<Self, DeboaError> {
        Ok(Self {
            client: Deboa::new(),
            base_url: url.into_url()?,
        })
    }

    pub fn client(&mut self) -> &mut Deboa {
        &mut self.client
    }

    pub fn get(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::get(format!("{}{}", self.base_url, path))
    }

    pub fn post(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::post(format!("{}{}", self.base_url, path))
    }

    pub fn put(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::put(format!("{}{}", self.base_url, path))
    }

    pub fn patch(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::patch(format!("{}{}", self.base_url, path))
    }

    pub fn delete(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::delete(format!("{}{}", self.base_url, path))
    }
}
