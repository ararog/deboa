/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.
pub mod dns;
pub mod http;
#[cfg(feature = "websockets")]
pub mod ws;
