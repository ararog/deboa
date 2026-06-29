/// DNS resolution for the Deboa HTTP client.
///
/// This module provides DNS resolution functionality for the Deboa HTTP client.pub(crate) mod dns;
pub mod dns;
pub(crate) mod http;
#[cfg(feature = "websockets")]
pub(crate) mod ws;
