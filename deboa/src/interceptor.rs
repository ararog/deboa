#![allow(unused_variables)]
use crate::{request::DeboaRequest, response::DeboaResponse};

/// DeboaInterceptor
///
/// Trait that define the middleware pattern for Deboa.
///
pub trait DeboaInterceptor: Send + Sync + 'static {
    /// This method is called before the request is sent.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    fn on_request(&self, request: &mut DeboaRequest) {}

    ///
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    fn on_response(&self, response: &mut DeboaResponse) {}
}

impl<T: DeboaInterceptor> DeboaInterceptor for Box<T> {
    fn on_request(&self, request: &mut DeboaRequest) {
        self.as_ref().on_request(request);
    }

    fn on_response(&self, response: &mut DeboaResponse) {
        self.as_ref().on_response(response);
    }
}
