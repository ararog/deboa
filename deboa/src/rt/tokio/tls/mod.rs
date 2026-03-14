#[cfg(feature = "tokio-native-tls")]
mod native;

#[cfg(feature = "tokio-rust-tls")]
mod rustls;

#[cfg(feature = "tokio-rust-tls")]
pub(crate) use rustls::*;

#[cfg(feature = "tokio-native-tls")]
pub(crate) use native::*;
