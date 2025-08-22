#![allow(unused_variables)]
use crate::{Deboa, DeboaResponse};

/// DeboaMiddleware
///
/// Trait that define the middleware pattern for Deboa.
///
pub trait DeboaMiddleware {
    /// This method is called before the request is sent.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    fn on_request(&self, request: &Deboa){}
    
    ///
    /// This method is called after the response is received.
    /// 
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    /// * `response` - The response that was received.
    ///
    fn on_response(&self, request: &Deboa, response: &mut DeboaResponse){}
}
