use deboa::{
    Deboa,
    errors::DeboaError,
    request::{DeboaRequest, DeboaRequestBuilder, IntoUrl},
};
use url::Url;

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
    pub fn new<T: IntoUrl>(url: T) -> Result<Self, DeboaError> {
        Ok(Self {
            client: Deboa::new(),
            base_url: url.into_url()?,
        })
    }

    pub fn client(&mut self) -> &mut Deboa {
        &mut self.client
    }

    fn url(&self, path: &str) -> Result<Url, DeboaError> {
        let url = self.base_url.join(path);
        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }
        Ok(url.unwrap())
    }

    pub fn get(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::get(self.url(path)?.as_str())
    }

    pub fn post(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::post(self.url(path)?.as_str())
    }

    pub fn put(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::put(self.url(path)?.as_str())
    }

    pub fn patch(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::patch(self.url(path)?.as_str())
    }

    pub fn delete(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::delete(self.url(path)?.as_str())
    }
}
