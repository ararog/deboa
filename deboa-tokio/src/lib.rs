#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use deboa::InnerClient;

use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::{dns::DefaultDnsResolver, http::conn::pool::HttpConnectionPool},
};
#[cfg(all(
    feature = "rust-tls",
    not(feature = "native-tls"),
    not(all(
        any(
            feature = "no-provider",
            feature = "default-rustls-provider",
            feature = "aws-lc-rustls-provider",
            feature = "ring-rustls-provider",
        ),
        any(
            feature = "default-rustls-verifier",
            feature = "webpki-rustls-verifier",
            feature = "platform-rustls-verifier"
        )
    ))
))]
compile_error!(
    "When enabling rust-tls features, you must also enable default-rustls-provider and default-rustls-verifier features."
);

#[cfg(all(feature = "native-tls", feature = "rust-tls"))]
compile_error!("You cannot enable native-tls and rust-tls features at the same time.");

#[cfg(all(not(any(feature = "native-tls", feature = "rust-tls")), feature = "http2"))]
compile_error!("HTTP2 requires native-tls or rust-tls support.");

#[cfg(all(feature = "native-tls", feature = "http3"))]
compile_error!("HTTP3 is not supported within native-tls runtime.");

#[cfg(not(any(feature = "http1", feature = "http2", feature = "http3")))]
compile_error!("At least one HTTP version feature must be enabled.");

/// Certificate management module for handling SSL/TLS certificates.
pub mod cert;
/// Internal module for HTTP and Websockets clients implementations.
pub mod client;
/// Internal runtime module for Tokio-based HTTP client implementation.
pub(crate) mod rt;

/// Inner client type with generic resolver.
pub type RuntimeClient<Resolver> =
    InnerClient<DeboaIdentity, DeboaCertificate, HttpConnectionPool, Resolver>;

/// Type alias for the Tokio-based HTTP client.
pub type Client = deboa::Client<RuntimeClient<DefaultDnsResolver>>;

/// Type alias for the custom Tokio-based HTTP client.
pub type CustomClient<Resolver> = deboa::Client<RuntimeClient<Resolver>>;
