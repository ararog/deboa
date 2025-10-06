pub struct ClientCert {
    cert: String,
    key: String,
    key_pw: String,
}

impl ClientCert {
    pub fn new(cert: String, key: String, key_pw: String) -> Self {
        ClientCert { cert, key, key_pw }
    }

    pub fn cert(&self) -> &str {
        &self.cert
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn key_pw(&self) -> &str {
        &self.key_pw
    }
}
