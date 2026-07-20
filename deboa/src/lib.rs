#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use crate::{
    errors::DeboaError,
    hook::{Hook, Hooked},
    request::{DeboaRequest, IntoRequest},
    response::DeboaResponse,
};
use std::{fmt::Display, future::Future, marker::PhantomData, pin::Pin, sync::Arc};

pub mod cache;
pub mod cert;
pub mod conn;
pub mod cookie;
pub mod dns;
pub mod errors;
pub mod form;
pub mod hook;
pub mod request;
pub mod response;
pub mod serde;
#[cfg(test)]
mod tests;
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

/// A pinned future that resolves to a result of type T or a VetisError
///
/// This is used for async operations that return a `VetisResult<T>`.
///
/// # Examples
///
/// ```rust,no_run
/// use deboa::({request::DeboaRequest, response::DeboaResponse}, DeboaFutureResult};
/// use std::pin::Pin;
///
/// let future: DeboaFutureResult<'static, DeboaResponse> = Box::pin(async move {
///     // Process request...
///     Ok(DeboaResponse::builder()
///         .status(http::StatusCode::OK)
///         .text("OK"))
/// });
/// ```
pub type DeboaFutureResult<'a, T> = Pin<Box<dyn Future<Output = Result<T>> + Send + Sync + 'a>>;

/// Type alias for boxed handler closures.
///
/// This represents an async function that takes a `Request` and returns
/// a `Response` or an error. Handlers are the core of request processing
/// in VeTiS virtual hosts.
///
/// # Examples
///
/// ```rust,no_run
/// use deboa::{request::DeboaRequest, response::DeboaResponse};
/// use std::pin::Pin;
///
/// let handler: HookFn = Box::new(|request: DeboaRequest| {
///     Box::pin(async move {
///         // Process request...
///         Ok(DeboaResponse::builder()
///             .status(http::StatusCode::OK)
///             .text("OK"))
///     })
/// });
/// ```
pub type HookFn<Req, Res> =
    Box<dyn Fn(Req, SharedHook<Req, Res>) -> DeboaFutureResult<'static, Res> + Send + Sync>;

/// Type alias for boxed next hook closures.
///
/// This represents an async function that takes a `Request` and returns
/// a `Response` or an error. Next hooks are used to chain multiple hooks
/// together in a pipeline.
///
/// # Examples
///
/// ```rust,no_run
/// use deboa::{request::DeboaRequest, response::DeboaResponse};
/// use std::pin::Pin;
///
/// let next_hook: NextHook = Box::new(|request: DeboaRequest| {
///     Box::pin(async move {
///         // Process request...
///         Ok(DeboaResponse::builder()
///             .status(http::StatusCode::OK)
///             .text("OK"))
///     })
/// });
/// ```
pub type SharedHook<Req, Res> = Arc<Box<dyn hook::Hook<Req, Res>>>;

#[derive(PartialEq, Debug, Clone)]
/// Enum that represents the HTTP version.
///
/// # Variants
///
/// * `Http1` - The HTTP/1.1 version.
/// * `Http2` - The HTTP/2 version.
/// * `Http3` - The HTTP/3 version.
pub enum HttpVersion {
    /// HTTP/1.1 version
    Http1,
    /// HTTP/2 version
    Http2,
    /// HTTP/3 version
    Http3,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpVersion::Http1 => write!(f, "HTTP/1.1"),
            HttpVersion::Http2 => write!(f, "HTTP/2"),
            HttpVersion::Http3 => write!(f, "HTTP/3"),
        }
    }
}

/// HTTP client trait
pub trait HttpClient {
    /// Execute a request
    fn execute<R>(&self, request: R) -> impl Future<Output = Result<DeboaResponse>>
    where
        R: IntoRequest;
}

/// Client manager struct
pub struct ClientManager<C> {
    hook: SharedHook<DeboaRequest, DeboaResponse>,
    _p: PhantomData<C>,
}

impl<C> ClientManager<C>
where
    C: HttpClient + Hook<DeboaRequest, DeboaResponse> + 'static,
{
    /// Create a new client manager with the given client
    pub fn new(client: C) -> Self {
        Self { hook: Arc::new(Box::new(client)), _p: PhantomData }
    }

    /// Set the hook
    pub fn hook(mut self, new_hook: HookFn<DeboaRequest, DeboaResponse>) -> Self {
        self.hook = Arc::new(Box::new(Hooked::new(new_hook, self.hook)));
        self
    }
}

impl<C> Default for ClientManager<C>
where
    C: HttpClient + Default + Hook<DeboaRequest, DeboaResponse> + 'static,
{
    fn default() -> Self {
        Self { hook: SharedHook::new(Box::new(C::default())), _p: PhantomData }
    }
}

impl<C> HttpClient for ClientManager<C>
where
    C: HttpClient + 'static,
{
    /// Execute a request
    async fn execute<R>(&self, request: R) -> Result<DeboaResponse>
    where
        R: IntoRequest,
    {
        self.hook
            .handle(
                request.into_request()?,
                self.hook
                    .next_hook(),
            )
            .await
    }
}
