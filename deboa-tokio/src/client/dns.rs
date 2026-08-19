//! DNS resolution for the Deboa HTTP client.
//!
//! This module provides DNS resolution functionality for the Deboa HTTP client.

use deboa::{
    dns::DnsResolver,
    errors::{DeboaError::Dns, DnsError},
};
use rand::seq::SliceRandom;
use std::net::IpAddr;
use tokio::net::lookup_host;

/// Default DNS resolver implementation using tokio::net::lookup_host
#[derive(Default, Clone)]
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    async fn resolve(&self, host: String, port: u16) -> deboa::Result<Vec<IpAddr>> {
        let hostname = format!("{}:{}", host, port);
        let addrs = lookup_host(hostname).await;
        if let Err(e) = addrs {
            return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
        }

        let mut ips: Vec<IpAddr> = addrs
            .unwrap()
            .map(|addr| addr.ip())
            .collect();
        ips.shuffle(&mut rand::rng());
        Ok(ips)
    }
}
