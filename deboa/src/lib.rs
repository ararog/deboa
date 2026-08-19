#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{
    cert::{Certificate, Identity},
    conn::{ConnectionConfig, HttpConnectionDispatcher, HttpConnectionPool},
    dns::DnsResolver,
    errors::{DeboaError, RequestError},
    request::{DeboaRequest, DeboaRequestBuilder, IntoRequest},
    response::DeboaResponse,
};
use async_lock::RwLock;
use log::info;
use std::{
    future::Future,
    net::{IpAddr, Ipv4Addr},
    ops::Shl,
    time::Duration,
};
use tackle::{Chain, Hook, HookFn};

pub mod cache;
pub mod cert;
pub mod conn;
pub mod cookie;
pub mod dns;
pub mod errors;
pub mod form;
pub mod request;
pub mod response;
pub mod serde;
#[cfg(test)]
pub mod tests;
pub mod url;

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

/// Type alias for test results
/// Convenience alias for handling test errors throughout the library.
///
/// # Examples
///
/// ```
/// use deboa::TestResult;
///
/// fn example() -> TestResult<String> {
///     Ok("success".to_string())
/// }
/// ```
pub type TestResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Type alias for Result<T, DeboaError>
///
/// This is a convenience alias for handling Deboa errors throughout the library.
///
/// # Examples
///
/// ```
/// use deboa::DeboaResult;
///
/// fn example() -> DeboaResult<String> {
///     Ok("success".to_string())
/// }
/// ```
///
/// # See Also
/// - [DeboaError](crate::errors::DeboaError)
pub type DeboaResult<T> = Result<T>;

/// HTTP client trait
pub trait HttpClient {
    /// Execute a request
    fn execute<R>(&self, request: R) -> impl Future<Output = Result<DeboaResponse>>
    where
        R: IntoRequest;
}

/// Client parameters struct
pub struct ClientBuilder<I, C, P, R> {
    inner: InnerClient<I, C, P, R>,
}

impl<I, C, P, R> ClientBuilder<I, C, P, R>
where
    I: Identity + Send + Clone + 'static,
    C: Certificate + Send + Clone + 'static,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Default + Send + 'static,
    R: DnsResolver + Default + Send + 'static,
{
    /// Set skip certificate verification
    pub fn skip_cert_verification(mut self, skip: bool) -> Self {
        self.inner
            .skip_cert_verification = skip;
        self
    }

    /// Set connection timeout
    pub fn connection_timeout(mut self, connection_timeout: Duration) -> Self {
        self.inner
            .connection_timeout = connection_timeout;
        self
    }

    /// Set request timeout
    pub fn request_timeout(mut self, request_timeout: Duration) -> Self {
        self.inner
            .request_timeout = request_timeout;
        self
    }

    /// Set certificate
    pub fn certificate(mut self, certificate: C) -> Self {
        self.inner
            .certificate = Some(certificate);
        self
    }

    /// Set identity
    pub fn identity(mut self, identity: I) -> Self {
        self.inner.identity = Some(identity);
        self
    }

    /// Set client bind address
    pub fn bind_addr(mut self, bind_addr: IpAddr) -> Self {
        self.inner.bind_addr = bind_addr;
        self
    }

    /// Set dns resolver
    pub fn dns_resolver(mut self, dns_resolver: R) -> Self {
        self.inner
            .dns_resolver = dns_resolver;
        self
    }

    /// Set connction pool
    pub fn connection_pool(mut self, pool: P) -> Self {
        self.inner.pool = RwLock::new(pool);
        self
    }

    /// Build the client
    pub fn build(self) -> Client<InnerClient<I, C, P, R>> {
        Client::from_inner(self.inner)
    }
}

/// Client struct
pub struct Client<H> {
    hook: H,
}

impl<H> Client<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>> + 'static,
{
    /// Initialize a client from hook
    pub fn new(inner: H) -> Self {
        Self { hook: inner }
    }

    /// Add a new hook to the chain
    pub fn chain<C, Hout>(self, chain: C) -> Client<Hout>
    where
        C: Chain<H, DeboaError, DeboaRequest, DeboaResponse, Hook = Hout>,
        Hout: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>> + 'static,
    {
        Client::new(chain.chain(self.hook))
    }

    /// Add a hook from a function
    pub fn chain_fn<F, Fut>(self, f: F) -> Client<HookFn<F, H>>
    where
        F: Fn(DeboaRequest, std::rc::Rc<H>) -> Fut + Send,
        Fut: Future<Output = Result<DeboaResponse>>,
    {
        Client::from_fn(HookFn::new(self.hook, f))
    }
}

impl<F, H> Client<HookFn<F, H>> {
    /// Initialize a client from hook
    pub fn from_fn(inner: HookFn<F, H>) -> Self {
        Self { hook: inner }
    }
}

impl<I, C, P, R> Client<InnerClient<I, C, P, R>>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool + Default + Send,
    R: DnsResolver + Default + Send,
{
    /// Create a client from inner client
    pub fn from_inner(inner: InnerClient<I, C, P, R>) -> Self {
        Self { hook: inner }
    }

    /// Returns a new builer
    pub fn builder() -> ClientBuilder<I, C, P, R> {
        ClientBuilder { inner: InnerClient::<I, C, P, R>::default() }
    }
}

///
/// Extension trait for Client to enable the `<<` operator for URL construction.
/// This allows for a more ergonomic way to create requests using the `<<` operator.
/// The operator creates a GET request with the provided URL.
///
/// # Examples
///
/// ``` rust,ignore
/// use deboa::{Client, Result};
/// use deboa_tokio::InnerClient;
///
/// #[tokio::main]
/// fn main() -> Result<()> {
///     let client = Client::<InnerClient>::default();
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
impl<H> Shl<&str> for &Client<H> {
    type Output = DeboaRequestBuilder;

    fn shl(self, other: &str) -> Self::Output {
        DeboaRequest::get(other).expect("Invalid URL!")
    }
}

impl<H> Default for Client<H>
where
    H: Hook<DeboaRequest, DeboaResponse> + Default,
{
    fn default() -> Self {
        Self { hook: H::default() }
    }
}

impl<H> HttpClient for Client<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    async fn execute<Req>(&self, request: Req) -> Result<DeboaResponse>
    where
        Req: IntoRequest,
    {
        self.hook
            .call(request.into_request()?)
            .await
    }
}

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
/// - Support for multiple HTTP protocols (HTTP/1.1, HTTP/2)
/// - Thread-safe and `Send` + `Sync`
///
/// # Examples
///
/// ## Basic Usage
///
/// ``` ignore
/// use deboa::{Result};
/// use deboa_tokio::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///   // Create a new client with default settings
///   let client = Client::default();
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
pub struct InnerClient<I, C, P, R> {
    connection_timeout: Duration,
    request_timeout: Duration,
    identity: Option<I>,
    certificate: Option<C>,
    skip_cert_verification: bool,
    pool: RwLock<P>,
    dns_resolver: R,
    bind_addr: IpAddr,
}

impl<I, C, P, R> InnerClient<I, C, P, R> {
    #[inline]
    /// Check if certificate verification is skipped.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if certificate verification is skipped, `false` otherwise.
    pub fn skip_cert_verification(&self) -> bool {
        self.skip_cert_verification
    }

    #[inline]
    /// Allow get request connection timeout at any time.
    ///
    /// # Returns
    ///
    /// * `Duration` - The timeout.
    ///
    pub fn connection_timeout(&self) -> Duration {
        self.connection_timeout
    }

    #[inline]
    /// Allow get request request timeout at any time.
    ///
    /// # Returns
    ///
    /// * `Duration` - The timeout.
    ///
    pub fn request_timeout(&self) -> Duration {
        self.request_timeout
    }

    /// Allow get connection pool at any time.
    ///
    /// # Returns
    ///
    /// * `Option<std::cell::Ref<'_, HttpConnectionPool>>` - The connection pool.
    ///
    #[inline]
    pub fn connection_pool(&self) -> &RwLock<P> {
        &self.pool
    }

    /// Allow get DNS resolver at any time.
    ///
    /// # Returns
    ///
    /// * `Arc<dyn DnsResolver>` - The DNS resolver.
    ///
    #[inline]
    pub fn dns_resolver(&self) -> &R {
        &self.dns_resolver
    }

    /// Allow get bind address at any time.
    ///
    /// # Returns
    ///
    /// * `IpAddr` - The bind address.
    ///
    #[inline]
    pub fn bind_addr(&self) -> IpAddr {
        self.bind_addr
    }

    /// Allow get certificate at any time.
    ///
    /// # Returns
    ///
    /// * `Option<Identity>` - The certificate.
    ///
    pub fn certificate(&self) -> &Option<C> {
        &self.certificate
    }

    /// Allow get identity at any time.
    ///
    /// # Returns
    ///
    /// * `Option<Identity>` - The identity.
    ///
    #[inline]
    pub fn identity(&self) -> &Option<I> {
        &self.identity
    }
}

impl<I, C, P, R> Default for InnerClient<I, C, P, R>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool + Default + Send,
    R: DnsResolver + Default + Send,
{
    fn default() -> Self {
        Self {
            bind_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(30),
            identity: None,
            certificate: None,
            skip_cert_verification: false,
            pool: RwLock::new(P::default()),
            dns_resolver: R::default(),
        }
    }
}

impl<I, C, P, R> Hook<DeboaRequest, DeboaResponse> for InnerClient<I, C, P, R>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
{
    type Result = Result<DeboaResponse>;
    type Error = DeboaError;

    async fn call(&self, request: DeboaRequest) -> Result<DeboaResponse> {
        info!("Building request: {} {}", request.method(), request.uri());

        let uri = request
            .uri()
            .clone();

        let Some(scheme) = uri.scheme_str() else {
            return Err(DeboaError::Request(RequestError::Send {
                message: "Missing scheme".to_string(),
            }));
        };

        let Some(host) = uri.host() else {
            return Err(DeboaError::Request(RequestError::Send {
                message: "Missing host".to_string(),
            }));
        };

        let port = uri
            .port_u16()
            .unwrap_or({
                match scheme {
                    "http" => 80,
                    "https" => 443,
                    _ => 80,
                }
            });

        let config = ConnectionConfig::builder()
            .scheme(scheme)
            .host(host)
            .port(port)
            .protocol_version(request.version())
            .identity(
                self.identity
                    .as_ref(),
            )
            .certificate(
                self.certificate
                    .as_ref(),
            )
            .skip_cert_verification(self.skip_cert_verification)
            .client_bind_addr(self.bind_addr)
            .build();

        let mut pool = self
            .pool
            .write()
            .await;

        let conn = pool
            .create_connection(&config, &self.dns_resolver)
            .await?;

        let request = request.body();

        let response = conn
            .send_request(request, self.request_timeout)
            .await?;

        Ok(response)
    }
}
