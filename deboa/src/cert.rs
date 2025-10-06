pub struct ClientCert {
    cert: String,
    pw: String,
}

impl ClientCert {
    pub fn new(cert: String, pw: String) -> Self {
        ClientCert { cert, pw }
    }

    pub fn cert(&self) -> &str {
        &self.cert
    }

    pub fn pw(&self) -> &str {
        &self.pw
    }
}
