#[cfg(any(feature = "http1", feature = "http2"))]
pub mod tcp;
#[cfg(feature = "http3")]
pub mod udp;

#[derive(Default)]
pub struct ServerConfig {
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    client_auth: Option<bool>,
}

impl ServerConfig {
    pub fn new(cert: Option<Vec<u8>>, key: Option<Vec<u8>>) -> Self {
        Self { cert, key, client_auth: None }
    }

    pub fn cert(&self) -> Option<&Vec<u8>> {
        self.cert.as_ref()
    }

    pub fn key(&self) -> Option<&Vec<u8>> {
        self.key.as_ref()
    }

    pub fn client_auth(&self) -> Option<bool> {
        self.client_auth
    }
}
