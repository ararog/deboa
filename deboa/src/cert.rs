pub struct ClientCert {
    cert: String,
    pw: String,
    ca: Option<String>,
}

impl ClientCert {
    /// Allow create a new ClientCert instance.
    ///
    /// # Arguments
    ///
    /// * `cert` - The client certificate.
    /// * `pw` - The client certificate password.
    /// * `ca` - The client certificate authority.
    ///
    /// # Returns
    ///
    /// * `ClientCert` - The new ClientCert instance.
    ///
    pub fn new(cert: String, pw: String, ca: Option<String>) -> Self {
        ClientCert { cert, pw, ca }
    }

    /// Allow get the client certificate.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate.
    ///
    pub fn cert(&self) -> &str {
        &self.cert
    }

    /// Allow get the client certificate password.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate password.
    ///
    pub fn pw(&self) -> &str {
        &self.pw
    }

    /// Allow get the client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The client certificate authority.
    ///
    pub fn ca(&self) -> Option<&str> {
        self.ca.as_deref()
    }
}
