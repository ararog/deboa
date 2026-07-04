use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use smol::net::resolve;
use std::net::IpAddr;

/// Default DNS resolver implementation using smol::net::resolve
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
            let hostname = format!("{}:{}", host, port);
            let addrs = resolve(hostname).await;
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
