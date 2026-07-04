use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use std::net::IpAddr;
use tokio::net::lookup_host;

/// Default DNS resolver implementation using tokio::net::lookup_host
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
            let hostname = format!("{}:{}", host, port);
            let addrs = lookup_host(hostname).await;
            if let Ok(addrs) = addrs {
                let ips: Vec<IpAddr> = addrs
                    .into_iter()
                    .map(|addr| addr.ip())
                    .collect();
                Ok(ips)
            } else {
                Err(Dns(DnsError::Resolve {
                    host,
                    message: addrs
                        .err()
                        .unwrap()
                        .to_string(),
                }))
            }
        };
        Box::pin(future)
    }
}
