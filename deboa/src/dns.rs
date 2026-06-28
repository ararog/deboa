//! DNS module for resolving hostnames to IP addresses.
use crate::Result;
use std::{future::Future, net::IpAddr, pin::Pin};

/// Type alias for DNS resolution future
pub type DnsResolverFuture = Pin<Box<dyn Future<Output = Result<Vec<IpAddr>>> + Send>>;

/// DNS resolver trait for resolving hostnames to IP addresses.
pub trait DnsResolver: Send + Sync + 'static {
    /// Resolves a hostname to a list of IP addresses.
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture;
}
