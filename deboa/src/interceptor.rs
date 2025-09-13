#![allow(unused_variables)]

use mockall::automock;

use crate::{errors::DeboaError, request::DeboaRequest, response::DeboaResponse};

/// DeboaInterceptor
///
/// Trait that define the middleware pattern for Deboa.
///
#[automock]
pub trait DeboaInterceptor: Send + Sync + 'static {
    ///
    /// This method is called before the request is sent. Please note if this method returns a response,
    /// the request will not be sent and the response will be returned.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `Result<Option<DeboaResponse>, DeboaError>` - The response that was received.
    ///
    fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>, DeboaError> {
        Ok(None)
    }

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
    fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>, DeboaError> {
        self.as_ref().on_request(request)
    }

    fn on_response(&self, response: &mut DeboaResponse) {
        self.as_ref().on_response(response);
    }
}
