use deboa::{
    Deboa,
    errors::DeboaError,
    request::{DeboaRequest, DeboaRequestBuilder},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Vamo {
    client: Deboa,
    base_url: String,
}

impl Vamo {
    pub fn new(url: &str) -> Self {
        Self {
            client: Deboa::new(),
            base_url: url.to_string(),
        }
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
