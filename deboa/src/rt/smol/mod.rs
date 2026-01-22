#[cfg(any(feature = "smol-rust-tls", feature = "smol-native-tls"))]
pub(crate) mod tls;

pub mod stream;

#[cfg(feature = "http2")]
pub(crate) mod executor;
