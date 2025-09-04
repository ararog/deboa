use crate::errors::DeboaError;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::client::conn::http1::SendRequest;
use hyper::{Request, Response};
use url::Url;

#[async_trait::async_trait]
pub trait Http1Connection: Send + Sync + 'static {
    /// Connects to the specified URL.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to connect to.
    ///
    /// # Returns
    ///
    /// A `Result` containing the connection or an error.
    ///
    async fn connect(url: Url) -> Result<BaseHttp1Connection, DeboaError>;
}

pub struct BaseHttp1Connection {
    url: Url,
    sender: SendRequest<Full<Bytes>>,
}

impl BaseHttp1Connection {
    /// Creates a new `BaseHttp1Connection` instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL for the connection.
    /// * `sender` - The sender for the connection.
    ///
    /// # Returns
    ///
    /// A new `BaseHttp1Connection` instance.
    ///
    pub fn new(url: Url, sender: SendRequest<Full<Bytes>>) -> Self {
        Self { url, sender }
    }

    /// Sets the URL for the connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL for the connection.
    ///
    /// # Returns
    ///
    /// A new `BaseHttp1Connection` instance.
    ///
    pub fn set_url(&mut self, url: Url) {
        self.url = url;
    }

    /// Gets the URL for the connection.
    ///
    /// # Returns
    ///
    /// The URL for the connection.
    ///
    pub fn get_url(&self) -> &Url {
        &self.url
    }

    /// Sends a request using the connection.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method for the request.
    /// * `headers` - The headers for the request.
    /// * `encodings` - The encodings for the request.
    /// * `body` - The body for the request.
    ///
    /// # Returns
    ///
    /// A `Result` containing the response or an error.
    ///
    pub async fn send_request(&mut self, request: Request<Full<Bytes>>) -> Result<Response<Incoming>, DeboaError> {
        let method = request.method().to_string();
        let response = self.sender.send_request(request).await;

        if let Err(err) = response {
            println!("Error: {err}");
            return Err(DeboaError::Request {
                host: self.url.host().unwrap().to_string(),
                path: self.url.path().to_string(),
                method,
                message: err.to_string(),
            });
        }

        Ok(response.unwrap())
    }
}
