pub(crate) mod plain;

#[cfg(all(
    any(feature = "rust-tls", feature = "native-tls"),
    any(feature = "http1", feature = "http2")
))]
pub(crate) mod tls;

pub(crate) use plain::*;
pub(crate) use tls::*;
