#[cfg(feature = "http1")]
pub(crate) mod http1;

#[cfg(feature = "http2")]
pub(crate) mod http2;

#[cfg(feature = "http3-smol")]
pub(crate) mod http3;

#[cfg(any(feature = "smol-rust-tls", feature = "smol-native-tls"))]
pub(crate) mod tls;

pub mod stream;

#[cfg(feature = "http2")]
pub(crate) mod executor;
