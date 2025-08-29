#![allow(unused_variables)]
use crate::{request::DeboaRequest, response::DeboaResponse};

/// DeboaMiddleware
///
/// Trait that define the middleware pattern for Deboa.
///
pub trait DeboaMiddleware: Send + Sync + 'static {
    /// This method is called before the request is sent.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    fn on_request<Req: DeboaRequest>(&self, request: &Req) {}

    ///
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    /// * `response` - The response that was received.
    ///
    fn on_response<Req: DeboaRequest, Res: DeboaResponse>(&self, request: &Req, response: &mut Res) {}
}
