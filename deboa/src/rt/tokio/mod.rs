#[cfg(feature = "http1")]
pub(crate) mod http1;
#[cfg(feature = "http2")]
pub(crate) mod http2;
#[cfg(feature = "http3-tokio")]
pub(crate) mod http3;

#[cfg(any(feature = "tokio-rust-tls", feature = "tokio-native-tls"))]
pub(crate) mod tls;

pub mod stream;
