use url::Url;

use crate::{errors::RequestError, DeboaError, Result};

/// Trait to convert a value into a Url.
pub trait IntoUrl: private::IntoUrlSealed {
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
    #[inline]
    fn parse_url(&self) -> Result<Url>
    where
        Self: AsRef<str>,
    {
        let url_ref = self.as_ref();
        if !url_ref.starts_with("http") && !url_ref.starts_with("ws") {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Scheme must be http or https or ws or wss".to_string(),
            }));
        }

        let url = Url::parse(url_ref);

        if url.is_err() {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Failed to parse url".to_string(),
            }));
        }

        Ok(url.unwrap())
    }
}

impl IntoUrl for Url {
    #[inline]
    fn into_url(self) -> Result<Url> {
        Ok(self)
    }
}

impl IntoUrl for &str {
    #[inline]
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

impl IntoUrl for &mut String {
    #[inline]
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

impl IntoUrl for &String {
    #[inline]
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

impl IntoUrl for String {
    #[inline]
    fn into_url(self) -> Result<Url> {
        self.parse_url()
    }
}

/// Sealed trait to prevent external implementation.
///
/// This is used to ensure that the `IntoUrl` trait can only be implemented
/// for types that are defined in this crate.
mod private {
    pub trait IntoUrlSealed {}
}

/// Implement the `IntoUrl` trait for `Url`, `&str`, `&mut String`, and `String`.
impl private::IntoUrlSealed for Url {}

impl private::IntoUrlSealed for &str {}

impl private::IntoUrlSealed for &String {}

impl private::IntoUrlSealed for &mut String {}

impl private::IntoUrlSealed for String {}
