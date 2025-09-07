#![allow(unused_variables)]
use crate::{Deboa, response::DeboaResponse};

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
    fn on_request(&self, request: &Deboa) {}

    ///
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    /// * `response` - The response that was received.
    ///
    fn on_response(&self, request: &Deboa, response: &mut DeboaResponse) {}
}

impl<T: DeboaMiddleware> DeboaMiddleware for Box<T> {
    fn on_request(&self, request: &Deboa) {
        self.as_ref().on_request(request);
    }

    fn on_response(&self, request: &Deboa, response: &mut DeboaResponse) {
        self.as_ref().on_response(request, response);
    }
}
