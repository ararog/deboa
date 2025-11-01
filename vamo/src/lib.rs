use deboa::{
    Deboa, Result, errors::{DeboaError, RequestError}, request::{DeboaRequest, DeboaRequestBuilder}, response::DeboaResponse, url::IntoUrl
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
    /// A Result containing the new Vamo instance.
    ///
    pub fn new<T: IntoUrl>(url: T) -> Result<Self> {
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
    /// A Result containing the URL with the given path.
    ///
    fn url(&self, path: &str) -> Result<Url> {
        let url = self.base_url.join(path);
        if let Err(e) = url {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
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
    /// A Result containing a DeboaRequestBuilder.
    ///
    pub fn get(&self, path: &str) -> Result<DeboaRequestBuilder> {
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
    /// A Result containing a DeboaRequestBuilder.
    ///
    pub fn post(&self, path: &str) -> Result<DeboaRequestBuilder> {
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
    /// A Result containing a DeboaRequestBuilder.
    ///
    pub fn put(&self, path: &str) -> Result<DeboaRequestBuilder> {
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
    /// A Result containing a DeboaRequestBuilder.
    ///
    pub fn patch(&self, path: &str) -> Result<DeboaRequestBuilder> {
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
    /// A Result containing a DeboaRequestBuilder.
    ///
    pub fn delete(&self, path: &str) -> Result<DeboaRequestBuilder> {
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
    /// A Result containing the DeboaResponse.
    ///
    pub async fn go(&mut self, mut request: DeboaRequest) -> Result<DeboaResponse> {
        let mut url = self.base_url.to_string();
        url.push_str(request.url().path());
        let result = request.set_url(url);
        if let Err(e) = result {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: e.to_string(),
            }));
        }

        self.client.execute(request).await
    }
}
