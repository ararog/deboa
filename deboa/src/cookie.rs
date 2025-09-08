use std::fmt;

pub struct DeboaCookie {
    name: String,
    value: String,
    expires: Option<String>,
    path: Option<String>,
    domain: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
}

impl DeboaCookie {
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

    ///
    /// # Arguments
    ///
    /// * `expires` - The cookie expires.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_expires(&mut self, expires: String) -> &mut Self {
        self.expires = Some(expires);
        self
    }

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

impl fmt::Display for DeboaCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={};", self.name, self.value)
    }
}
