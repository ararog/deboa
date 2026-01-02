#[cfg(feature = "http1")]
pub(crate) mod http1;

#[cfg(feature = "http2")]
pub(crate) mod http2;

#[cfg(feature = "http3-smol")]
pub(crate) mod http3;

pub mod stream;

pub(crate) mod executor;
