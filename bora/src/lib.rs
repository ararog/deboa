use deboa::{
    Deboa,
    request::{DeboaRequest, DeboaRequestBuilder},
};

#[cfg(test)]
mod tests;

pub struct Bora {
    client: Deboa,
    base_url: String,
}

impl Bora {
    pub fn new(url: &str) -> Self {
        Self {
            client: Deboa::new(),
            base_url: url.to_string(),
        }
    }

    pub fn client(&mut self) -> &mut Deboa {
        &mut self.client
    }

    pub fn get(&self, path: &str) -> DeboaRequestBuilder {
        DeboaRequest::get(&format!("{}{}", self.base_url, path))
    }

    pub fn post(&self, path: &str) -> DeboaRequestBuilder {
        DeboaRequest::post(&format!("{}{}", self.base_url, path))
    }

    pub fn put(&self, path: &str) -> DeboaRequestBuilder {
        DeboaRequest::put(&format!("{}{}", self.base_url, path))
    }

    pub fn patch(&self, path: &str) -> DeboaRequestBuilder {
        DeboaRequest::patch(&format!("{}{}", self.base_url, path))
    }

    pub fn delete(&self, path: &str) -> DeboaRequestBuilder {
        DeboaRequest::delete(&format!("{}{}", self.base_url, path))
    }
}
