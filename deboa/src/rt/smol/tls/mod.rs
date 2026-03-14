#[cfg(feature = "smol-native-tls")]
mod native;

#[cfg(feature = "smol-rust-tls")]
mod rustls;

#[cfg(feature = "smol-rust-tls")]
pub(crate) use rustls::*;

#[cfg(feature = "smol-native-tls")]
pub(crate) use native::*;
