/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.pub(crate) mod dns;
pub mod dns;
pub mod http;
pub mod tls;
#[cfg(feature = "websockets")]
pub mod ws;
