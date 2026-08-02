use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use rand::seq::SliceRandom;
use std::net::IpAddr;
use tokio::net::lookup_host;

/// Default DNS resolver implementation using tokio::net::lookup_host
#[derive(Default, Clone)]
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
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
        };
        Box::pin(future)
    }
}
