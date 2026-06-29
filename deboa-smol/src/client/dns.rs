use std::net::IpAddr;

use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use rand::{rng, seq::SliceRandom};
use smol::net::resolve;

/// Default DNS resolver implementation using smol::net::resolve
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
            let hostname = format!("{}:{}", host, port);
            let addrs = resolve(hostname).await;
            if let Ok(addrs) = addrs {
                let mut ips: Vec<IpAddr> = addrs
                    .into_iter()
                    .map(|addr| addr.ip())
                    .collect();
                ips.shuffle(&mut rng());
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
