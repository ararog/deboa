use std::fmt;

use cookie::{Cookie, time::OffsetDateTime};

#[derive(Clone)]
pub struct DeboaCookie {
    name: String,
    value: String,
    expires: Option<OffsetDateTime>,
    path: Option<String>,
    domain: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
}

impl DeboaCookie {
    /// Create a new cookie.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name.
    /// * `value` - The cookie value.
    ///
    /// # Returns
    ///
    /// * `DeboaCookie` - The new cookie.
    ///
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            expires: None,
            path: None,
            domain: None,
            secure: None,
            http_only: None,
        }
    }

    /// Get the cookie name.
    ///
    /// # Returns
    ///
    /// * `&str` - The cookie name.
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Set the cookie expires.
    ///
    /// # Arguments
    ///
    /// * `expires` - The cookie expires.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_expires(&mut self, expires: OffsetDateTime) -> &mut Self {
        self.expires = Some(expires);
        self
    }

    /// Set the cookie path.
    ///
    /// # Arguments
    ///
    /// * `path` - The cookie path.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    /// Set the cookie domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The cookie domain.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_domain(&mut self, domain: String) -> &mut Self {
        self.domain = Some(domain);
        self
    }

    /// Set the cookie secure.
    ///
    /// # Arguments
    ///
    /// * `secure` - The cookie secure.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = Some(secure);
        self
    }

    /// Set the cookie http only.
    ///
    /// # Arguments
    ///
    /// * `http_only` - The cookie http only.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = Some(http_only);
        self
    }
}

impl fmt::Debug for DeboaCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeboaCookie")
            .field("name", &self.name)
            .field("value", &self.value)
            .field("expires", &self.expires)
            .field("path", &self.path)
            .field("domain", &self.domain)
            .field("secure", &self.secure)
            .field("http_only", &self.http_only)
            .finish()
    }
}

impl From<DeboaCookie> for Cookie<'_> {
    fn from(deboa_cookie: DeboaCookie) -> Self {
        let mut cookie = Self::new(deboa_cookie.name, deboa_cookie.value);
        if let Some(expires) = deboa_cookie.expires {
            cookie.set_expires(expires);
        }
        if let Some(path) = deboa_cookie.path {
            cookie.set_path(path);
        }
        if let Some(domain) = deboa_cookie.domain {
            cookie.set_domain(domain);
        }
        if let Some(secure) = deboa_cookie.secure {
            cookie.set_secure(secure);
        }
        if let Some(http_only) = deboa_cookie.http_only {
            cookie.set_http_only(http_only);
        }
        cookie
    }
}
