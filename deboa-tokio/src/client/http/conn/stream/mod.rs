/// TLS module for runtime-specific TLS implementations.
#[cfg(all(
    any(feature = "rust-tls", feature = "native-tls"),
    any(feature = "http1", feature = "http2")
))]
pub(crate) mod tls;

/// Plain module for runtime-specific plain implementations.
pub(crate) mod plain;

pub(crate) use plain::*;
pub(crate) use tls::*;
