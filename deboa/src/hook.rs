//! Hook module
//!
//! This module provides functionality for creating and managing hooks in the Deboa HTTP client.
//!
//! Hooks are functions that can be executed before or after a request is made. They can be used to modify the request or response, or to perform additional actions.
//!
//! # Examples
//!
//! ```rust,no_run
//! use deboa::hook::hook_fn;
//!
//! let hook = hook_fn(|request| {
//!     Box::pin(async move {
//!         // Process request...
//!         Ok(DeboaResponse::builder()
//!             .status(http::StatusCode::OK)
//!             .text("OK"))
//!     })
//! });
//! ```
use crate::{DeboaFutureResult, DeboaResult, HookFn, SharedHook};
use std::{future::Future, sync::Arc};

/// Creates an interceptor function from a function.
///
/// # Examples
///
/// ```rust,no_run
/// use deboa::hook::hook_fn;
///
/// let hook = hook_fn(|request| {
///     Box::pin(async move {
///         // Process request...
///         Ok(DeboaResponse::builder()
///             .status(http::StatusCode::OK)
///             .text("OK"))
///     })
/// });
/// ```
pub fn hook_fn<F, Fut, Req, Res>(f: F) -> HookFn<Req, Res>
where
    F: Fn(Req, SharedHook<Req, Res>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = DeboaResult<Res>> + Send + Sync + 'static,
{
    Box::new(move |req, next| Box::pin(f(req, next)))
}

/// Trait for defining hooks that can be executed before or after a request.
///
/// # Type Parameters
///
/// * `Req` - The type of the request.
/// * `Res` - The type of the response.
///
/// # Examples
///
/// ```rust,no_run
/// use deboa::{
///   DeboaFutureResult,
///   HookFn,
///   hook::Hook,
///   request::DeboaRequest,
///   response::DeboaResponse
/// };
///
/// struct MyHook<Req, Res> {
///     handler: HookFn,
///     next: NextHook,
/// };
///
/// impl Hooker<DeboaRequest, DeboaResponse> for MyHook {
///     fn call(&self, request: DeboaRequest) -> DeboaFutureResult<'_, DeboaResponse> {
///         Box::pin(async move {
///             // Process request...
///             Ok(DeboaResponse::builder()
///                 .status(http::StatusCode::OK)
///                 .text("OK"))
///         })
///     }
/// }
/// ```
pub trait Hook<Req, Res>: Send + Sync {
    /// Executes the hook with the given request.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to process.
    /// * `next` - The next hook in the chain.
    ///
    /// # Returns
    ///
    /// A future that resolves to a result containing the response or an error.
    fn handle(&self, request: Req, next: SharedHook<Req, Res>) -> DeboaFutureResult<'_, Res>
    where
        Self: Sync + Send;

    /// Executes the hook with the given request and no next hook.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to process.
    ///
    /// # Returns
    ///
    /// A future that resolves to a result containing the response or an error.
    fn call(&self, request: Req) -> DeboaFutureResult<'_, Res>
    where
        Self: Sync + Send;

    /// Returns the next hook in the chain.
    ///
    /// # Returns
    ///
    /// The next hook in the chain.
    fn next_hook(&self) -> SharedHook<Req, Res>;
}

/// Struct that represents a hook and its next hook in the chain.
pub struct Hooked<Req, Res> {
    hook_fn: HookFn<Req, Res>,
    next: SharedHook<Req, Res>,
}

impl<Req, Res> Hooked<Req, Res> {
    /// Creates a new hook with the given handler and next hook.
    ///
    /// # Arguments
    ///
    /// * `hook_fn` - The handler for this hook.
    /// * `next` - The next hook in the chain.
    ///
    /// # Returns
    ///
    /// A new hook with the given handler and next hook.
    pub fn new(hook_fn: HookFn<Req, Res>, next: SharedHook<Req, Res>) -> Self {
        Self { hook_fn, next }
    }
}

impl<Req, Res> Hook<Req, Res> for Hooked<Req, Res> {
    fn handle(&self, request: Req, next: SharedHook<Req, Res>) -> DeboaFutureResult<'_, Res> {
        (self.hook_fn)(request, next)
    }

    fn call(&self, request: Req) -> DeboaFutureResult<'_, Res> {
        self.handle(request, self.next.clone())
    }

    fn next_hook(&self) -> SharedHook<Req, Res> {
        self.next.clone()
    }
}

/// A hook that does nothing.
pub struct NoopHook<Req, Res> {
    _phantom: std::marker::PhantomData<(Req, Res)>,
}

impl<Req, Res> NoopHook<Req, Res>
where
    Req: Send + Sync + 'static,
    Res: Send + Sync + 'static,
{
    /// Creates a new noop hook.
    ///
    /// # Returns
    ///
    /// A new noop hook.
    pub fn noop() -> SharedHook<Req, Res> {
        Arc::new(Box::new(Self { _phantom: std::marker::PhantomData }))
    }
}

impl<Req, Res> Default for NoopHook<Req, Res> {
    fn default() -> Self {
        Self { _phantom: std::marker::PhantomData }
    }
}

impl<Req, Res> Hook<Req, Res> for NoopHook<Req, Res>
where
    Req: Send + Sync + 'static,
    Res: Send + Sync + 'static,
{
    fn handle(&self, _request: Req, _next: SharedHook<Req, Res>) -> DeboaFutureResult<'_, Res> {
        unimplemented!()
    }

    fn call(&self, _request: Req) -> DeboaFutureResult<'_, Res>
    where
        Self: Sync + Send,
    {
        unimplemented!()
    }

    fn next_hook(&self) -> SharedHook<Req, Res> {
        unimplemented!()
    }
}
