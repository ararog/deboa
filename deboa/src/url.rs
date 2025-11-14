use url::Url;

use crate::{errors::RequestError, DeboaError, Result};

/// Trait to convert a value into a Url.
pub trait IntoUrl {
    /// Convert the value into a Url.
    ///
    /// # Returns
    ///
    /// * `Result<Url>` - The url.
    ///
    fn into_url(self) -> Result<Url>;
    /// Parse a string into a Url.
    ///
    /// # Returns
    ///
    /// * `Result<Url>` - The url.
    ///
    fn parse_url(&self) -> Result<Url>
    where
        Self: AsRef<str>,
    {
        if !self
            .as_ref()
            .starts_with("http")
            && !self
                .as_ref()
                .starts_with("ws")
        {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Scheme must be http or https or ws or wss".to_string(),
            }));
        }

        let url = Url::parse(self.as_ref());

        if url.is_err() {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Failed to parse url".to_string(),
            }));
        }

        Ok(url.unwrap())
    }
}

impl IntoUrl for Url {
    fn into_url(self) -> Result<Url> {
        Ok(self)
    }
}

impl IntoUrl for &str {
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

impl IntoUrl for &mut String {
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

impl IntoUrl for String {
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}
