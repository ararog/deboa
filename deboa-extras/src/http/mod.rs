#[cfg(any(
    feature = "json",
    feature = "xml",
    feature = "msgpack",
    feature = "yaml",
    feature = "flex",
    feature = "cbor"
))]
pub mod serde;

#[cfg(feature = "sse")]
pub mod sse;

#[cfg(feature = "utils")]
pub mod utils;
