//! # Deboa - Core API Documentation
//!
//! Hello, and welcome to the core Deboa API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//!
//! Depend on `deboa` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = "0.1.0"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ``` rust,ignore
//! use deboa::{Client, Result, errors::DeboaError, request::DeboaRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut client = Client::builder()
//!         .build();
//!
//!     let response = DeboaRequest::get("https://httpbin.org/get")?
//!         .send_with(&mut client)
//!         .await?;
//!
//!     println!("Response: {:#?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! To avoid compiling unused dependencies, Deboa feature-gates optional
//! functionality, some enabled by default:
//!
//! | Feature           | Default? | Description                                             |
//! |-------------------|----------|---------------------------------------------------------|
//! | `tokio_rt`        | Yes      | Support tokio runtime (enabled by default).             |
//! | `smol_rt`         | No       | Support smol runtime.                                   |
//! | `http1`           | No       | Support for HTTP/1.                                     |
//! | `http2`           | Yes      | Support for HTTP/2 (enabled by default).                |
//! | `http3`           | No       | Support for HTTP/3.                                     |
//! | `http3-smol`      | No       | Support for HTTP/3 on Smol.                             |
//! | `tokio-rust-tls`  | Yes      | Support for tokio-rust-tls (enabled by default).        |
//! | `tokio-native-tls`| No       | Support for tokio-native-tls.                           |
//! | `smol-rust-tls`   | No       | Support for smol-rust-tls.                              |
//! | `smol-native-tls` | No       | Support for smol-native-tls.                            |
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.1.0", features = ["tokio_rt", "http2", "tokio-rust-tls"] }
//! ```
//!
//! Conversely, HTTP/2 can be disabled:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.1.0", default-features = false }
//! ```
//!

#[cfg(all(feature = "tokio-native-tls", feature = "http3"))]
compile_error!("HTTP3 is not supported within tokio-native-tls runtime.");

#[cfg(all(feature = "smol-native-tls", feature = "http3"))]
compile_error!("HTTP3 is not supported within smol-native-tls runtime.");

#[cfg(all(feature = "tokio-rt", feature = "smol-rt"))]
compile_error!("Only one runtime feature can be enabled at a time.");

#[cfg(not(any(feature = "http1", feature = "http2", feature = "http3")))]
compile_error!("At least one HTTP version feature must be enabled.");

pub(crate) const MAX_ERROR_MESSAGE_SIZE: usize = 50000;

#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
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

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
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

use cfg_if::cfg_if;

use futures::{stream, TryStreamExt};
use http_body::Frame;

#[cfg(feature = "tokio-rt")]
use tokio::sync::{RwLock, RwLockWriteGuard};

#[cfg(feature = "smol-rt")]
use smol::lock::RwLock;

#[cfg(feature = "tokio-rt")]
use tokio_util::io::ReaderStream;

use std::fmt::{Debug, Display};
use std::ops::Shl;

use bytes::Bytes;
use http::{header, HeaderValue, Request};
use http_body_util::{BodyExt, StreamBody};
use log::{error, info};

use crate::cert::{Certificate, Identity};

use crate::client::conn::{ConnectionConfig, ConnectionFactory};

#[cfg(feature = "tokio-rt")]
use crate::errors::IoError;

use crate::catcher::DeboaCatcher;
use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::errors::{DeboaError, RequestError};
use crate::request::{DeboaRequest, IntoRequest};
use crate::response::DeboaResponse;

pub use async_trait::async_trait;

#[cfg(feature = "tokio-rt")]
pub type File = tokio::fs::File;
#[cfg(feature = "smol-rt")]
pub type File = smol::fs::File;

pub mod cache;
pub mod catcher;
pub mod cert;
pub mod client;
pub mod cookie;
pub mod errors;
pub mod form;
pub mod fs;
pub mod request;
pub mod response;
pub mod rt;
pub mod url;

#[cfg(test)]
mod tests;

/// Type alias for Result<T, DeboaError>
/// Convenience alias for handling Deboa errors throughout the library.
///
/// # Examples
///
/// ```
/// use deboa::Result;
///
/// fn example() -> Result<String> {
///     Ok("success".to_string())
/// }
/// ```
///
/// # See Also
/// - [DeboaError](crate::errors::DeboaError)
pub type Result<T> = std::result::Result<T, DeboaError>;

///
/// Extension trait for Client to enable the `<<` operator for URL construction.
/// This allows for a more ergonomic way to create requests using the `<<` operator.
/// The operator creates a GET request with the provided URL.
///
/// # Examples
///
/// ``` rust,ignore
/// use deboa::{Client, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = Client::new();
///     let request = &client << "https://httpbin.org/get";
///     // do something with the request
///     Ok(())
/// }
/// ```
///
/// # Notes
/// - This implementation is primarily for convenience and ergonomics
/// - For more complex request configurations, use the full DeboaRequest API
/// - The `<<` operator is a shorthand for creating GET requests
impl Shl<&str> for &Client {
    type Output = DeboaRequest;

    fn shl(self, other: &str) -> Self::Output {
        DeboaRequest::get(other)
            .expect("Invalid URL!")
            .build()
            .expect("Invalid request!")
    }
}

#[derive(PartialEq, Debug, Clone)]
/// Enum that represents the HTTP version.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 version.
/// * `Http2` - The HTTP/2 version.
pub enum HttpVersion {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
    #[cfg(feature = "http3")]
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => write!(f, "HTTP/1.1"),
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}

#[deprecated(note = "DeboaBuilder is now ClientBuilder")]
pub type DeboaBuilder = ClientBuilder;

/// A builder for configuring and creating a new `Deboa` client instance.
///
/// This builder allows you to configure various aspects of the HTTP client before
/// constructing it. You can set timeouts, configure protocols, add error handlers,
/// and more.
///
/// # Examples
///
/// ``` rust,ignore
/// use deboa::{Client, Result, HttpVersion};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   let client = Client::builder()
///     .connection_timeout(30)  // 30 seconds
///     .request_timeout(10)     // 10 seconds
///     .protocol(HttpVersion::Http2)  // Use HTTP/2
///     .build();
///
///   // Use the client to make requests...
///   Ok(())
/// }
/// ```
///
/// # Default Configuration
///
/// - Connection timeout: 30 seconds
/// - Request timeout: 30 seconds
/// - Protocol: HTTP/1.1
/// - No client certificates
/// - No custom error catchers
pub struct ClientBuilder {
    connection_timeout: u64,
    request_timeout: u64,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    skip_cert_verification: bool,
    pool: Option<RwLock<HttpConnectionPool>>,
}

impl ClientBuilder {
    /// Sets the maximum duration to wait when connecting to a server.
    ///
    /// This timeout affects the initial TCP connection establishment. If the server
    /// doesn't respond within this duration, the connection will fail with a timeout error.
    ///
    /// # Arguments
    ///
    /// * `connection_timeout` - The timeout in seconds.
    ///
    /// # Examples
    ///
    /// ``` rust, no_run
    /// use deboa::Client;
    /// let builder = Client::builder()
    ///     .connection_timeout(10);  // 10 seconds
    /// ```
    ///
    /// # Note
    /// A value of 0 means no timeout (not recommended in production).
    #[inline]
    pub fn connection_timeout(mut self, connection_timeout: u64) -> Self {
        self.connection_timeout = connection_timeout;
        self
    }

    /// Sets the maximum duration for the entire HTTP request/response cycle.
    ///
    /// This includes connection time, request writing, server processing, and response reading.
    /// If the entire operation takes longer than this duration, it will be aborted.
    ///
    /// # Arguments
    ///
    /// * `request_timeout` - The timeout in seconds.
    ///
    /// # Examples
    ///
    /// ``` rust, no_run
    /// use deboa::Client;
    /// let builder = Client::builder()
    ///     .request_timeout(30);  // 30 seconds
    /// ```
    ///
    /// # Note
    /// A value of 0 means no timeout (not recommended in production).
    #[inline]
    pub fn request_timeout(mut self, request_timeout: u64) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    /// Configures a client certificate for mutual TLS authentication.
    ///
    /// This is used when the server requires client certificate authentication.
    /// The certificate should be in PEM format and include both the certificate
    /// and private key.
    ///
    /// # Arguments
    ///
    /// * `client_cert` - The client certificate configuration.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Client, Result, Identity};
    ///
    /// #[tokio::main]
    ///
    /// async fn main() -> Result<()> {
    ///     let cert = Identity::from_pem_file("client.pem")?;
    ///     let builder = Client::builder()
    ///         .set_identity(cert);
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn client_cert(mut self, client_cert: Identity) -> Self {
        self.identity = Some(client_cert);
        self
    }

    /// Configures a client certificate for mutual TLS authentication.
    ///
    /// This is used when the server requires client certificate authentication.
    /// The certificate should be in PEM format and include both the certificate
    /// and private key.
    ///
    /// # Arguments
    ///
    /// * `identity` - The client certificate file.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Client, Identity, Result};
    ///
    /// #[tokio::main]
    ///
    /// async fn main() -> Result<()> {
    ///     let cert = Identity::new("client.pem", Some("pw"))?;
    ///     let builder = Client::builder()
    ///         .identity(cert);
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn identity(mut self, identity: Identity) -> Self {
        self.identity = Some(identity);
        self
    }

    /// Configures a ca certificate.
    ///
    /// # Arguments
    ///
    /// * `certificate` - The ca certificate file.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Client, Certificate, Result};
    ///
    /// #[tokio::main]
    ///
    /// async fn main() -> Result<()> {
    ///     let cert = Certificate::new("client.pem")?;
    ///     let builder = Client::builder()
    ///         .certificate(cert);
    ///     Ok(())
    /// }
    /// ```
    #[inline]
    pub fn certificate(mut self, certificate: Certificate) -> Self {
        self.certificate = Some(certificate);
        self
    }

    /// Adds an error handler for specific types of errors.
    ///
    /// Catchers are called when an error occurs during request execution.
    /// They can be used to implement custom error handling logic, such as
    /// automatic retries, logging, or error transformation.
    ///
    /// # Arguments
    ///
    /// * `catcher` - A function or closure that handles specific error types.
    ///
    /// # Examples
    ///
    /// ## Automatic Retries
    ///
    /// ```ignore
    /// use deboa::{Client, Result, catcher::DeboaCatcher, request::DeboaRequest, response::DeboaResponse};
    ///
    /// struct AddAuthorization;
    ///
    /// #[deboa::async_trait]
    /// impl DeboaCatcher for AddAuthorization {
    ///     async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
    ///         println!("Request: {:?}", request.url());
    ///         Ok(None)
    ///     }
    ///
    ///     async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
    ///         println!("Response: {:?}", response.status());
    ///         Ok(())
    ///     }
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let client = Client::builder()
    ///         .catch(AddAuthorization)
    ///         .build();
    ///     Ok(())
    /// }
    /// ```
    pub fn catch<C: DeboaCatcher>(mut self, catcher: C) -> Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catcher));
        } else {
            self.catchers = Some(vec![Box::new(catcher)]);
        }
        self
    }

    /// Sets the HTTP protocol version to use for requests.
    ///
    /// By default, the client will use HTTP/1.1. You can choose to use HTTP/2
    /// for better performance, especially for multiple requests to the same server.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The HTTP protocol version to use.
    ///
    /// # Examples
    ///
    /// ``` rust,ignore
    /// use deboa::{Client, HttpVersion};
    ///
    /// let builder = Client::builder()
    ///     .protocol(HttpVersion::Http2);  // Use HTTP/2
    /// ```
    ///
    /// # Note
    /// The actual protocol version used may be negotiated with the server
    /// during the TLS handshake.
    #[inline]
    pub fn protocol(mut self, protocol: HttpVersion) -> Self {
        self.protocol = protocol;
        self
    }

    /// Skip certificate verification.
    ///
    /// # Arguments
    ///
    /// * `skip` - Whether to skip certificate verification.
    ///
    /// # Examples
    ///
    /// ``` rust, no_run
    /// use deboa::Client;
    ///
    /// let builder = Client::builder()
    ///     .skip_cert_verification(true);  // Skip certificate verification
    /// ```
    ///
    /// # Warning
    /// This should only be used in development or testing environments.
    /// Never use this in production as it makes your application vulnerable to man-in-the-middle attacks.
    ///
    /// # Note
    /// This setting affects all connections made by the client.
    /// It is recommended to use this only for testing purposes.
    ///
    /// # Safety
    /// This function bypasses SSL certificate validation, which can expose your application to security risks.
    /// Only use this in controlled environments where you trust all network traffic.
    ///
    #[inline]
    pub fn skip_cert_verification(mut self, skip: bool) -> Self {
        self.skip_cert_verification = skip;
        self
    }

    /// Set a connection pool.
    ///
    /// # Arguments
    ///
    /// * `pool` - The connection pool to use.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    ///
    /// # Example
    ///
    /// ```compile_fail
    /// use deboa::Client;
    ///
    /// let client = Client::builder()
    ///     .pool(HttpConnectionPool::default())
    ///     .build();
    /// ```
    #[inline]
    pub fn pool(mut self, pool: HttpConnectionPool) -> Self {
        self.pool = Some(RwLock::new(pool));
        self
    }

    /// Constructs a new `Deboa` client with the configured settings.
    ///
    /// This consumes the builder and returns a new `Deboa` instance that can
    /// be used to make HTTP requests.
    ///
    /// # Returns
    ///
    /// A new `Deboa` client instance.
    ///
    /// # Examples
    ///
    /// ``` rust,ignore
    /// use deboa::{Client, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let client = Client::builder()
    ///     .connection_timeout(10)
    ///     .request_timeout(30)
    ///     .build();
    ///
    ///   // client is now ready to make requests
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This method may panic if the underlying HTTP client cannot be created
    /// with the specified configuration.
    #[inline]
    pub fn build(self) -> Client {
        Client {
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            identity: self.identity,
            certificate: self.certificate,
            catchers: self.catchers,
            protocol: self.protocol,
            skip_cert_verification: self.skip_cert_verification,
            pool: self.pool,
        }
    }
}

#[deprecated(note = "Deboa is now Client")]
pub type Deboa = Client;

/// The main HTTP client for making requests.
///
/// `Deboa` is a flexible and efficient HTTP client that supports both synchronous
/// and asynchronous operations. It provides a builder pattern for configuration
/// and supports features like connection pooling, timeouts, and custom error handling.
///
/// # Features
///
/// - Connection pooling for better performance
/// - Configurable timeouts
/// - Custom error handling with catchers
/// - Support for multiple HTTP protocols (HTTP/1.1, HTTP/2)
/// - Thread-safe and `Send` + `Sync`
///
/// # Examples
///
/// ## Basic Usage
///
/// ``` ignore
/// use deboa::{Client, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   // Create a new client with default settings
///   let client = Client::new();
///
///   // Or configure with custom settings
///   let client = Client::builder()
///     .connection_timeout(10)  // 10 seconds
///     .request_timeout(30)     // 30 seconds
///     .build();
///   Ok(())
/// }
/// ```
///
/// # Thread Safety
///
/// `Deboa` implements `Send` and `Sync`, making it safe to share between threads.
/// The connection pool is managed internally and optimized for concurrent access.
///
/// # Performance
///
/// - Connection pooling reduces latency for repeated requests to the same host
/// - Automatic connection reuse when possible
/// - Configurable timeouts prevent hanging requests
pub struct Client {
    connection_timeout: u64,
    request_timeout: u64,
    identity: Option<Identity>,
    certificate: Option<Certificate>,
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    skip_cert_verification: bool,
    pool: Option<RwLock<HttpConnectionPool>>,
}

impl AsRef<Client> for Client {
    fn as_ref(&self) -> &Client {
        self
    }
}

impl AsMut<Client> for Client {
    fn as_mut(&mut self) -> &mut Client {
        self
    }
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("connection_timeout", &self.connection_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("protocol", &self.protocol)
            .finish()
    }
}

pub(crate) const fn default_protocol() -> HttpVersion {
    cfg_if! {
      if #[cfg(feature = "http1")] {
          HttpVersion::Http1
      } else if #[cfg(feature = "http2")] {
          HttpVersion::Http2
      } else {
          HttpVersion::Http3
      }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            connection_timeout: 0,
            request_timeout: 0,
            identity: None,
            certificate: None,
            catchers: None,
            protocol: default_protocol(),
            skip_cert_verification: false,
            pool: None,
        }
    }
}

impl Client {
    /// Creates a new `Deboa` instance with default settings.
    ///
    /// This is equivalent to calling `Deboa::builder().build()` and provides
    /// a quick way to get started with sensible defaults.
    ///
    /// # Default Configuration
    ///
    /// - Connection timeout: 30 seconds
    /// - Request timeout: 30 seconds
    /// - Protocol: HTTP/1.1
    /// - No client certificates
    /// - No custom error catchers
    ///
    /// # Returns
    ///
    /// A new `Client` instance with default settings.
    ///
    /// # Examples
    ///
    /// ``` rust,ignore
    /// use deboa::{Client, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let client = Client::new();
    ///   // client is ready to make requests
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # Deprecated
    ///
    /// This method is deprecated and will be removed in a future release. Use `Client::default()` instead.
    ///
    /// # See Also
    ///
    /// - [`Client::builder()`] for custom configuration
    /// - [`Client::default()`] for the same functionality via the `Default` trait
    #[deprecated(note = "Use Client::default() instead", since = "0.0.9")]
    pub fn new() -> Self {
        Self {
            connection_timeout: 0,
            request_timeout: 0,
            identity: None,
            certificate: None,
            catchers: None,
            protocol: default_protocol(),
            skip_cert_verification: false,
            pool: None,
        }
    }

    /// Allow create a new Deboa instance.
    ///
    /// # Returns
    ///
    /// * `ClientBuilder` - The new ClientBuilder instance.
    ///
    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            connection_timeout: 0,
            request_timeout: 0,
            identity: None,
            certificate: None,
            catchers: None,
            protocol: default_protocol(),
            skip_cert_verification: false,
            pool: None,
        }
    }

    /// Check if certificate verification is skipped.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if certificate verification is skipped, `false` otherwise.
    #[inline]
    pub fn skip_cert_verification(&self) -> bool {
        self.skip_cert_verification
    }

    /// Allow get protocol at any time.
    ///
    /// # Returns
    ///
    /// * `&HttpVersion` - The protocol.
    ///
    #[inline]
    pub fn protocol(&self) -> &HttpVersion {
        &self.protocol
    }

    /// Allow get request connection timeout at any time.
    ///
    /// # Returns
    ///
    /// * `u64` - The timeout.
    ///
    #[inline]
    pub fn connection_timeout(&self) -> u64 {
        self.connection_timeout
    }

    /// Allow get connection pool at any time.
    ///
    /// # Returns
    ///
    /// * `Option<std::cell::Ref<'_, HttpConnectionPool>>` - The connection pool.
    ///
    #[inline]
    #[cfg(feature = "tokio-rt")]
    pub async fn connection_pool(&self) -> Option<&tokio::sync::RwLock<HttpConnectionPool>> {
        self.pool.as_ref()
    }

    #[inline]
    #[cfg(feature = "smol-rt")]
    pub async fn connection_pool(&self) -> Option<&smol::lock::RwLock<HttpConnectionPool>> {
        self.pool.as_ref()
    }

    /// Allow get request request timeout at any time.
    ///
    /// # Returns
    ///
    /// * `u64` - The timeout.
    ///
    #[inline]
    pub fn request_timeout(&self) -> u64 {
        self.request_timeout
    }

    /// Allow get client certificate at any time.
    ///
    /// # Returns
    ///
    /// * `Option<ClientCert>` - The client certificate.
    ///
    #[inline]
    #[deprecated(note = "Use identity instead", since = "0.0.8")]
    pub fn client_cert(&self) -> Option<&Identity> {
        self.identity
            .as_ref()
    }

    /// Allow get identity at any time.
    ///
    /// # Returns
    ///
    /// * `Option<Identity>` - The identity.
    ///
    #[inline]
    pub fn identity(&self) -> Option<&Identity> {
        self.identity
            .as_ref()
    }

    /// Executes an HTTP request and returns the response.
    ///
    /// This is the primary method for making HTTP requests. It handles the entire
    /// request/response lifecycle, including retries, error handling, and response processing.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to execute. This can be:
    ///   - A string URL (for GET requests)
    ///   - A `DeboaRequest` instance (for more control)
    ///   - Any type that implements `IntoRequest`
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `Ok(DeboaResponse)` - The successful HTTP response
    /// - `Err(DeboaError)` - If the request fails or encounters an error
    ///
    /// # Examples
    ///
    /// ## Simple GET Request
    ///
    /// ```rust,ignore
    /// use deboa::{Client, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut client = Client::new();
    ///   let response = client.execute("https://httpbin.org/get").await?;
    ///   println!("Status: {}", response.status());
    ///   println!("Body: {}", response.text().await?);
    ///   Ok(())
    /// }
    /// ```
    ///
    /// ## POST Request with JSON Body
    ///
    /// ```rust,ignore
    /// use deboa::{Client, Result, request::post};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut client = Client::new();
    ///   let response = client
    ///     .execute(
    ///         post("https://httpbin.org/post")
    ///             .text("text")?
    ///     )
    ///     .await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # Error Handling
    ///
    /// The method will automatically handle:
    /// - Network errors
    /// - Timeouts (if configured)
    /// - Invalid responses
    /// - Status code errors (unless configured otherwise)
    ///
    /// # Retries
    ///
    /// By default, failed requests are not automatically retried. To enable retries:
    ///
    /// ```rust,ignore
    /// use deboa::{Client, Result, request::get};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///   let mut client = Client::new();
    ///   let request = get("https://example.com").retries(3); // Retry up to 3 times
    ///   let response = client.execute(request).await?;
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # Panics
    /// - If the request is invalid
    /// - If the response is a non-success status code
    ///
    /// # Performance
    ///
    /// - Uses connection pooling for better performance
    /// - Automatically reuses connections when possible
    /// - Supports HTTP/1.1, HTTP/2 and HTTP/3
    pub async fn execute<R>(&self, request: R) -> Result<DeboaResponse>
    where
        R: IntoRequest,
    {
        let request = request.into_request()?;

        let url = request
            .as_ref()
            .url();
        let mut uri = url
            .path()
            .to_string();
        if let Some(query) = url.query() {
            uri.push('?');
            uri.push_str(query);
        }

        let method = request
            .as_ref()
            .method();

        info!("Building request: {} {}", method, uri);
        let mut builder = Request::builder()
            .uri(uri)
            .method(
                method
                    .to_string()
                    .as_str(),
            );
        {
            let req_headers = builder
                .headers_mut()
                .unwrap();

            request
                .as_ref()
                .headers()
                .into_iter()
                .fold(&mut *req_headers, |acc, (key, value)| {
                    acc.insert(key, value.into());
                    acc
                });

            if let Some(deboa_cookies) = request
                .as_ref()
                .cookies()
            {
                let mut cookies = Vec::<String>::new();

                for cookie in deboa_cookies.values() {
                    cookies.push(cookie.to_string());
                }

                if let Ok(cookie_header) = HeaderValue::from_str(&cookies.join("; ")) {
                    req_headers.insert(header::COOKIE, cookie_header);
                }
            }
        }

        #[cfg(feature = "tokio-rt")]
        let request = if let Some(file) = request.file() {
            let file = File::open(file)
                .await
                .map_err(|e| DeboaError::Io(IoError::File { message: e.to_string() }))?;
            let content = ReaderStream::new(file).map_ok(Frame::data);
            let body = StreamBody::new(content);
            builder.body(body.boxed())
        } else {
            let all_bytes = Bytes::from(
                request
                    .as_ref()
                    .raw_body()
                    .to_vec(),
            );
            let content = stream::iter(vec![Ok(all_bytes)]).map_ok(Frame::data);
            let body = StreamBody::new(content);
            builder.body(body.boxed())
        };

        #[cfg(feature = "smol-rt")]
        let request = {
            let all_bytes = Bytes::from(
                request
                    .as_ref()
                    .raw_body()
                    .to_vec(),
            );
            let content = stream::iter(vec![Ok(all_bytes)]).map_ok(Frame::data);
            let body = StreamBody::new(content);
            builder.body(body.boxed())
        };

        if let Err(err) = request {
            error!("Failed to send request: {}", err);
            return Err(DeboaError::Request(RequestError::Send {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let request = request.unwrap();

        let scheme = url.scheme();

        let host = url
            .host_str()
            .unwrap_or("localhost");

        let (port, is_secure) = if let Some(port) = url.port() {
            (port, scheme == "https" || scheme == "wss")
        } else {
            match scheme {
                "http" | "ws" => (80, false),
                "https" | "wss" => (443, true),
                _ => panic!("Unsupported scheme: {}", scheme),
            }
        };

        let config = ConnectionConfig::builder()
            .is_secure(is_secure)
            .host(host)
            .port(port)
            .protocol(
                self.protocol
                    .clone(),
            )
            .identity(
                self.identity
                    .clone(),
            )
            .certificate(
                self.certificate
                    .clone(),
            )
            .skip_cert_verification(self.skip_cert_verification)
            .build();

        let response = if let Some(pool) = &self.pool {
            #[cfg(feature = "tokio-rt")]
            let mut pool = RwLockWriteGuard::map(pool.write().await, |f| f);
            #[cfg(feature = "smol-rt")]
            let mut pool = pool.write().await;

            let conn = pool
                .create_connection(&config)
                .await?;

            conn.send_request(url.clone(), request)
                .await?
        } else {
            let mut conn = ConnectionFactory::create_connection(&self.protocol, &config).await?;
            conn.send_request(url.clone(), request)
                .await?
        };

        Ok(response)
    }
}
