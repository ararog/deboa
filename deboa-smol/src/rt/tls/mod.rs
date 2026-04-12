#[cfg(feature = "native-tls")]
mod native;

#[cfg(feature = "rust-tls")]
mod rustls;

#[cfg(feature = "rust-tls")]
pub(crate) use rustls::*;

#[cfg(feature = "native-tls")]
pub(crate) use native::*;
