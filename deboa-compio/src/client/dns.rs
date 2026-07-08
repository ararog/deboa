use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
    errors::{DeboaError::Dns, DnsError},
};
use rand::seq::SliceRandom;
use std::net::{IpAddr, ToSocketAddrs};

/// Default DNS resolver implementation using smol::net::resolve
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
            let hostname = format!("{}:{}", host, port);
            let addrs = hostname.to_socket_addrs();
            if let Err(e) = addrs {
                return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
            };

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
