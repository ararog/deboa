pub struct ClientCert {
    cert: String,
    pw: String,
    ca: Option<String>,
}

impl ClientCert {
    pub fn new(cert: String, pw: String, ca: Option<String>) -> Self {
        ClientCert { cert, pw, ca }
    }

    pub fn cert(&self) -> &str {
        &self.cert
    }

    pub fn pw(&self) -> &str {
        &self.pw
    }

    pub fn ca(&self) -> Option<&str> {
        self.ca.as_deref()
    }
}
