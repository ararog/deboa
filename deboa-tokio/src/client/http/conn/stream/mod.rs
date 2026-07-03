/// Plain module for runtime-specific plain implementations.
pub(crate) mod plain;
pub(crate) use plain::*;

/// TLS module for runtime-specific TLS implementations.
#[cfg(all(
    any(feature = "rust-tls", feature = "native-tls"),
    any(feature = "http1", feature = "http2", feature = "http3")
))]
pub(crate) mod tls;
#[cfg(all(
    any(feature = "rust-tls", feature = "native-tls"),
    any(feature = "http1", feature = "http2", feature = "http3")
))]
pub(crate) use tls::*;
