#[cfg(feature = "compio-native-tls")]
mod native;

#[cfg(feature = "compio-rust-tls")]
mod rustls;

#[cfg(feature = "compio-rust-tls")]
pub(crate) use rustls::*;

#[cfg(feature = "compio-native-tls")]
pub(crate) use native::*;
