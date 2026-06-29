#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
/// Errors module
pub mod errors;
/// HTTP module
pub mod http;
/// Serde module
#[cfg(any(
    feature = "json",
    feature = "xml",
    feature = "msgpack",
    feature = "yaml",
    feature = "flex",
    feature = "cbor"
))]
pub mod serde;
#[cfg(test)]
mod tests;
