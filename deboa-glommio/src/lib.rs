#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#[cfg(all(
    feature = "rust-tls",
    not(feature = "native-tls",),
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

#[cfg(feature = "native-tls")]
compile_error!(
    "deboa-glommio does not support native-tls: `async-native-tls` picks its runtime by feature \
     (runtime-tokio / runtime-smol) and has no glommio binding. Use the rust-tls feature."
);

#[cfg(feature = "http3")]
compile_error!(
    "deboa-glommio does not support HTTP/3 yet: it needs a `quinn::Runtime` implementation for \
     glommio, which does not exist. Use http1 or http2."
);

#[cfg(feature = "websockets")]
compile_error!("deboa-glommio does not support websockets yet.");

#[cfg(not(any(feature = "http1", feature = "http2")))]
compile_error!("At least one HTTP version feature must be enabled.");

use deboa::InnerClient;

use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::{dns::DefaultDnsResolver, http::conn::pool::HttpConnectionPool},
};

#[cfg(feature = "rust-tls")]
#[inline]
pub(crate) fn alpn() -> Vec<Vec<u8>> {
    vec![
        #[cfg(feature = "http2")]
        b"h2".to_vec(),
        #[cfg(feature = "http1")]
        b"http/1.1".to_vec(),
        #[cfg(feature = "http3")]
        b"h3".to_vec(),
    ]
}

#[cfg(feature = "native-tls")]
#[inline]
pub(crate) fn alpn() -> &'static [&'static str] {
    &[
        #[cfg(feature = "http2")]
        "h2",
        #[cfg(feature = "http1")]
        "http/1.1",
        #[cfg(feature = "http3")]
        "h3",
    ]
}

/// Certificate management module for handling SSL/TLS certificates.
pub mod cert;
/// Internal module for HTTP and Websockets clients implementations.
pub mod client;
/// Internal runtime module for Smol-based HTTP client implementation.
pub(crate) mod rt;

/// Inner client type with generic resolver.
pub type RuntimeClient<Resolver> =
    InnerClient<DeboaIdentity, DeboaCertificate, HttpConnectionPool, Resolver>;

/// Type alias for the Tokio-based HTTP client.
pub type Client = deboa::Client<RuntimeClient<DefaultDnsResolver>>;

/// Type alias for the custom Tokio-based HTTP client.
pub type CustomClient<Resolver> = deboa::Client<RuntimeClient<Resolver>>;
