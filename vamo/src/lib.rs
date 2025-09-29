use deboa::{
    errors::DeboaError, request::{DeboaRequest, DeboaRequestBuilder, IntoUrl}, response::DeboaResponse, Deboa
};
use url::Url;

pub mod resource;

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
    /// Creates a new Vamo instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL.
    ///
    /// # Returns
    ///
    /// A Result containing the new Vamo instance or a DeboaError.
    ///
    pub fn new<T: IntoUrl>(url: T) -> Result<Self, DeboaError> {
        Ok(Self {
            client: Deboa::new(),
            base_url: url.into_url()?,
        })
    }

    /// Returns a mutable reference to the Deboa client.
    ///
    /// # Returns
    ///
    /// A mutable reference to the Deboa client.
    ///
    pub fn client(&mut self) -> &mut Deboa {
        &mut self.client
    }

    /// Returns a Result containing the URL with the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing the URL with the given path or a DeboaError.
    ///
    fn url(&self, path: &str) -> Result<Url, DeboaError> {
        let url = self.base_url.join(path);
        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }
        Ok(url.unwrap())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a GET request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder or a DeboaError.
    ///
    pub fn get(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::get(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a POST request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder or a DeboaError.
    ///
    pub fn post(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::post(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a PUT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder or a DeboaError.
    ///
    pub fn put(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::put(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder or a DeboaError.
    ///
    pub fn patch(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::patch(self.url(path)?.as_str())
    }

    /// Returns a Result containing a DeboaRequestBuilder for a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to append to the base URL.
    ///
    /// # Returns
    ///
    /// A Result containing a DeboaRequestBuilder or a DeboaError.
    ///
    pub fn delete(&self, path: &str) -> Result<DeboaRequestBuilder, DeboaError> {
        DeboaRequest::delete(self.url(path)?.as_str())
    }

    /// Executes a DeboaRequest.
    ///
    /// # Arguments
    ///
    /// * `request` - The DeboaRequest to execute.
    ///
    /// # Returns
    ///
    /// A Result containing the DeboaResponse or a DeboaError.
    ///
    pub async fn go(&mut self, mut request: DeboaRequest) -> Result<DeboaResponse, DeboaError> {
        let url = self.url(request.url().path())?;
        println!("URL: {url}");
        let result = request.set_url(url);
        if let Err(e) = result {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }
        
        self.client.execute(request).await
    }
}
