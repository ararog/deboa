#[cfg(any(feature = "http1", feature = "http2"))]
pub mod tcp;
#[cfg(feature = "http3")]
pub mod udp;

#[derive(Default)]
pub struct ServerConfig {
    pub cert: Option<String>,
    pub key: Option<String>,
}

impl ServerConfig {
    pub fn new(cert: Option<String>, key: Option<String>) -> Self {
        Self { cert, key }
    }
}
