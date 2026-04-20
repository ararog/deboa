#![allow(unused_variables)]
//! This module define the catcher API for Deboa.
//!
//! Catchers are called when an error occurs during request execution.
//! They can be used to implement logic, such as logging, inject headers, etc.
//!
//! # Examples
//!
//! ```ignore
//! use deboa::{Result, catcher::DeboaCatcher, request::DeboaRequest, response::DeboaResponse};
//!
//! struct TestMonitor;
//!
//! #[deboa::async_trait]
//! impl DeboaCatcher for TestMonitor {
//!     async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
//!         println!("Request: {:?}", request.url());
//!         Ok(None)
//!     }
//!
//!     async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
//!         println!("Response: {:?}", response.status());
//!         Ok(())
//!     }
//! }
//!
//! // Create a client with middleware
//! let client = deboa::Deboa::builder()
//!     .catch(TestMonitor)
//!     .build();
//! ```
//!
use async_trait::async_trait;
use mockall::automock;

use crate::{request::DeboaRequest, response::DeboaResponse, Result};

/// DeboaCatcher
///
/// Trait that define the middleware pattern for Deboa. Keep in mind that
/// It is called before the request is sent and after the response is received.
/// Use it with caution and keep number of catchers low for better performance.
///
#[automock]
#[async_trait]
pub trait DeboaCatcher: Send + Sync + 'static {
    ///
    /// This method is called before the request is sent. Please note if this method returns a response,
    /// the request will not be sent and the response will be returned. It is advised to use bare minimum
    /// logic here to avoid performance issues.
    ///
    /// # Arguments
    ///
    /// * `request` - The request that was sent.
    ///
    /// # Returns
    ///
    /// * `Result<Option<DeboaResponse>>` - The response that was received.
    ///
    async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>>;

    ///
    /// This method is called after the response is received.
    ///
    /// # Arguments
    ///
    /// * `response` - The response that was received.
    ///
    async fn on_response(&self, response: &mut DeboaResponse) -> Result<()>;
}

#[async_trait]
impl<T: DeboaCatcher> DeboaCatcher for Box<T> {
    async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
        self.as_ref()
            .on_request(request)
            .await
    }

    async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
        self.as_ref()
            .on_response(response)
            .await
    }
}
