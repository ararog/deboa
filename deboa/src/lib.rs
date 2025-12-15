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
//! deboa = "0.0.8"
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ```rust,no_run
//! use deboa::{Deboa, Result, errors::DeboaError, request::DeboaRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let mut deboa = Deboa::builder()
//!         .build();
//!
//!     let response = DeboaRequest::get("https://httpbin.org/get")?
//!         .send_with(&mut deboa)
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
//! | Feature         | Default? | Description                                             |
//! |-----------------|----------|---------------------------------------------------------|
//! | `tokio_rt`      | Yes      | Support tokio runtime (enabled by default).             |
//! | `smol_rt`       | No       | Support smol runtime.                                   |
//! | `http1`         | Yes      | Support for HTTP/1 (enabled by default).                |
//! | `http2`         | Yes      | Support for HTTP/2 (enabled by default).                |
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.8", features = ["tokio_rt", "http1", "http2"] }
//! ```
//!
//! Conversely, HTTP/2 can be disabled:
//!
//! ```toml
//! [dependencies]
//! deboa = { version = "0.0.8", default-features = false }
//! ```
//!

#[cfg(all(feature = "tokio-rt", feature = "smol-rt"))]
compile_error!("Only one runtime feature can be enabled at a time.");

#[cfg(not(any(feature = "http1", feature = "http2")))]
compile_error!("At least one HTTP version feature must be enabled.");

pub(crate) const MAX_ERROR_MESSAGE_SIZE: usize = 50000;

use std::fmt::{Debug, Display};

use std::ops::Shl;

use bytes::Bytes;
use http::{header, HeaderValue, Request, Response};
use http_body_util::Full;
use hyper::body::Incoming;
use log::{error, info};

use crate::cert::{ClientCert, Identity};
use crate::client::conn::http::{DeboaConnection, DeboaHttpConnection};

use crate::catcher::DeboaCatcher;
use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::errors::DeboaError;
use crate::request::{DeboaRequest, IntoRequest};
use crate::response::{DeboaResponse, IntoBody};
use crate::url::IntoUrl;

pub use async_trait::async_trait;

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
/// ```
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
            .expect("Invalid url!")
            .build()
            .expect("Could not build request!")
    }
}

#[derive(PartialEq, Debug)]
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
/// ``` rust, no_run
/// use deboa::{Deboa, HttpVersion};
/// use std::time::Duration;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///   let client = deboa::Deboa::builder()
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
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
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
    /// use deboa::{Client, ClientCert};
    ///
    /// #[tokio::main]
    ///
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let cert = ClientCert::from_pem_file("client.pem")?;
    ///     let builder = Client::builder()
    ///         .client_cert(cert);
    ///     Ok(())
    /// }
    /// ```
    pub fn client_cert(mut self, client_cert: ClientCert) -> Self {
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
    /// use deboa::{Client, Identity};
    ///
    /// #[tokio::main]
    ///
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let cert = Identity::from_pem_file("client.pem")?;
    ///     let builder = Client::builder()
    ///         .identity(cert);
    ///     Ok(())
    /// }
    /// ```
    pub fn identity(mut self, identity: Identity) -> Self {
        self.identity = Some(identity);
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
    /// ## Basic Error Logging
    ///
    /// ``` compile_fail
    /// use deboa::Client;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let builder = Client::builder()
    ///         .catch(|e: std::io::Error| {
    ///             eprintln!("Network error: {}", e);
    ///             Ok(())  // Continue execution
    ///         });
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ## Automatic Retries
    ///
    /// ```
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
    /// ``` rust, no_run
    /// use deboa::{Client, HttpVersion};
    ///
    /// let builder = Client::builder()
    ///     .protocol(HttpVersion::Http2);  // Use HTTP/2
    /// ```
    ///
    /// # Note
    /// The actual protocol version used may be negotiated with the server
    /// during the TLS handshake.
    pub fn protocol(mut self, protocol: HttpVersion) -> Self {
        self.protocol = protocol;
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
    /// ``` rust, no_run
    /// use deboa::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    pub fn build(self) -> Client {
        Client {
            connection_timeout: self.connection_timeout,
            request_timeout: self.request_timeout,
            identity: self.identity,
            catchers: self.catchers,
            protocol: self.protocol,
            pool: HttpConnectionPool::new(),
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
/// ``` rust,no_run
/// use deboa::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    catchers: Option<Vec<Box<dyn DeboaCatcher>>>,
    protocol: HttpVersion,
    pool: HttpConnectionPool,
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
    /// ``` rust,no_run
    /// use deboa::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   let client = Client::new();
    ///   // client is ready to make requests
    ///   Ok(())
    /// }
    /// ```
    ///
    /// # See Also
    ///
    /// - [`Client::builder()`] for custom configuration
    /// - [`Client::default()`] for the same functionality via the `Default` trait
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Client {
            connection_timeout: 0,
            request_timeout: 0,
            identity: None,
            catchers: None,
            protocol: HttpVersion::Http1,
            pool: HttpConnectionPool::new(),
        }
    }

    /// Allow create a new Deboa instance.
    ///
    /// # Returns
    ///
    /// * `ClientBuilder` - The new ClientBuilder instance.
    ///
    pub fn builder() -> ClientBuilder {
        ClientBuilder {
            connection_timeout: 0,
            request_timeout: 0,
            identity: None,
            catchers: None,
            protocol: HttpVersion::Http1,
        }
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

    /// Allow change protocol at any time.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol to be used.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_protocol(&mut self, protocol: HttpVersion) -> &mut Self {
        self.protocol = protocol;
        self
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

    /// Allow change request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_connection_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connection_timeout = timeout;
        self
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

    /// Allow change request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_request_timeout(&mut self, timeout: u64) -> &mut Self {
        self.request_timeout = timeout;
        self
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

    /// Allow change client certificate at any time.
    ///
    /// # Arguments
    ///
    /// * `client_cert` - The client certificate to be used.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    #[deprecated(note = "Use set_identity instead", since = "0.0.8")]
    pub fn set_client_cert(&mut self, client_cert: Option<ClientCert>) -> &mut Self {
        self.identity = client_cert;
        self
    }

    /// Allow change identity at any time.
    ///
    /// # Arguments
    ///
    /// * `identity` - The identity to be used.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn set_identity(&mut self, identity: Option<Identity>) -> &mut Self {
        self.identity = identity;
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
    /// ## Basic Error Logging
    ///
    /// ```compile_fail
    /// use deboa::Client;
    /// use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// let builder = Client::builder()
    ///     .catch(|e: std::io::Error| {
    ///         eprintln!("Network error: {}", e);
    ///         Ok(())  // Continue execution
    ///     });
    /// # Ok(()) }
    /// ```
    ///
    /// ## Automatic Retries
    ///
    /// ```compile_fail
    /// use deboa::Client;
    /// use std::error::Error;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn Error>> {
    /// let builder = Client::builder()
    ///     .catch(|e: std::io::Error| {
    ///         if e.kind() == std::io::ErrorKind::TimedOut {
    ///             eprintln!("Request timed out, will retry...");
    ///             // Return error to trigger retry logic
    ///             Err(Box::new(e))
    ///         } else {
    ///             // For other errors, continue with the error
    ///             Ok(())
    ///         }
    ///     });
    /// # Ok(()) }
    /// ```
    ///
    /// # Notes
    /// - Multiple catchers can be added for different error types
    /// - Catchers are called in the order they are added
    /// - If a catcher returns `Ok(())`, error handling continues to the next catcher
    /// - If a catcher returns `Err(e)`, error propagation stops and the error is returned
    /// - The default error handler will be called if no catcher handles the error
    /// # Returns
    ///
    /// * `&mut Self` - The Deboa instance.
    ///
    pub fn catch<C: DeboaCatcher>(&mut self, catcher: C) -> &mut Self {
        if let Some(catchers) = &mut self.catchers {
            catchers.push(Box::new(catcher));
        } else {
            self.catchers = Some(vec![Box::new(catcher)]);
        }
        self
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
    /// ```rust,no_run
    /// use deboa::Client;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    /// ```compile_fail
    /// use deboa::{Client, request::post};
    /// use serde_json::json;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    /// ```compile_fail
    /// use deboa::{Client, request::get};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    /// - Supports HTTP/1.1 and HTTP/2
    pub async fn execute<R>(&mut self, request: R) -> Result<DeboaResponse>
    where
        R: IntoRequest,
    {
        let mut request = request.into_request()?;

        if let Some(catchers) = &self.catchers {
            let mut response = None;
            for catcher in catchers {
                response = catcher
                    .on_request(request.as_mut())
                    .await?;
            }

            if let Some(response) = response {
                let mut new_response = response;
                for catcher in catchers {
                    catcher
                        .on_response(new_response.as_mut())
                        .await?;
                }
                return Ok(new_response);
            }
        }

        let mut retry_count: u32 = 0;
        let response = loop {
            let response = self
                .send_request(request.as_mut())
                .await;
            if let Err(err) = response {
                if retry_count == request.retries() {
                    error!("Request failed after {} retries: {}", retry_count, err);
                    break Err(err);
                }
                #[cfg(feature = "tokio-rt")]
                tokio::time::sleep(tokio::time::Duration::from_secs(2_u32.pow(retry_count) as u64))
                    .await;
                #[cfg(feature = "smol-rt")]
                smol::Timer::after(std::time::Duration::from_secs(2_u32.pow(retry_count) as u64))
                    .await;
                retry_count += 1;
                continue;
            }

            let response = response.unwrap();

            if response
                .status()
                .is_redirection()
            {
                let location = response
                    .headers()
                    .get(header::LOCATION);
                info!(
                    "Redirecting to {}",
                    location
                        .unwrap()
                        .to_str()
                        .unwrap()
                );
                if let Some(location) = location {
                    let location = location
                        .to_str()
                        .unwrap();
                    let result = request
                        .as_mut()
                        .set_url(location);
                    if let Err(err) = result {
                        break Err(err);
                    }
                }
                continue;
            }

            break Ok(response);
        };

        let res_url = request
            .url()
            .to_string();
        let mut response = self
            .process_response(res_url, response?)
            .await?;

        if let Some(catchers) = &self.catchers {
            for catcher in catchers {
                catcher
                    .on_response(response.as_mut())
                    .await?;
            }
        }

        Ok(response)
    }

    /// Allow send a request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to be sent.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Incoming>>` - The response.
    ///
    async fn send_request<R>(&mut self, request: &R) -> Result<Response<Incoming>>
    where
        R: AsRef<DeboaRequest>,
    {
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

        let request = builder.body(Full::new(Bytes::from(
            request
                .as_ref()
                .raw_body()
                .to_vec(),
        )));
        if let Err(err) = request {
            error!("Failed to send request: {}", err);
            return Err(DeboaError::Request(errors::RequestError::Send {
                url: url.to_string(),
                method: method.to_string(),
                message: err.to_string(),
            }));
        }

        let request = request.unwrap();

        let conn = self
            .pool
            .create_connection(url, &self.protocol, &self.identity)
            .await?;
        match conn {
            #[cfg(feature = "http1")]
            DeboaConnection::Http1(ref mut conn) => {
                conn.send_request(request)
                    .await
            }
            #[cfg(feature = "http2")]
            DeboaConnection::Http2(ref mut conn) => {
                conn.send_request(request)
                    .await
            }
        }
    }

    /// Allow process a response.
    ///
    /// # Arguments
    ///
    /// * `response` - The response to be processed.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    async fn process_response<U>(
        &self,
        url: U,
        response: Response<Incoming>,
    ) -> Result<DeboaResponse>
    where
        U: IntoUrl,
    {
        let response = response.map(|body| body.into_body());
        let response = DeboaResponse::new(url.into_url()?, response);
        Ok(response)
    }
}
