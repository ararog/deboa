//! Serde module for HTTP operations

/// JSON serializer/deserializer
#[cfg(feature = "json")]
pub mod json;

/// MessagePack serializer/deserializer
#[cfg(feature = "msgpack")]
pub mod msgpack;

/// XML serializer/deserializer
#[cfg(feature = "xml")]
pub mod xml;

/// YAML serializer/deserializer
#[cfg(feature = "yaml")]
pub mod yaml;

/// Flexible serializer/deserializer
#[cfg(feature = "flex")]
pub mod flex;

/// CBOR serializer/deserializer
#[cfg(feature = "cbor")]
pub mod cbor;
