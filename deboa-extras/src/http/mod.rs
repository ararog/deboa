/// HTTP module
#[cfg(any(
    feature = "json",
    feature = "xml",
    feature = "msgpack",
    feature = "yaml",
    feature = "flex",
    feature = "cbor"
))]
pub mod serde;

/// SSE module
#[cfg(feature = "sse")]
pub mod sse;

/// Utils module
#[cfg(feature = "utils")]
pub mod utils;
