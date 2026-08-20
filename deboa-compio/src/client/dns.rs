use compio::net::ToSocketAddrsAsync;
use deboa::{
    dns::DnsResolver,
    errors::{DeboaError::Dns, DnsError},
};
use rand::seq::SliceRandom;
use std::net::IpAddr;

#[derive(Default, Clone)]
/// Default DNS resolver implementation using smol::net::resolve
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    async fn resolve(&self, host: String, port: u16) -> deboa::Result<Vec<IpAddr>> {
        let hostname = format!("{}:{}", host, port);
        let addrs = hostname
            .to_socket_addrs_async()
            .await;
        if let Err(e) = addrs {
            return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
        };

        let mut ips: Vec<IpAddr> = addrs
            .unwrap()
            .map(|addr| addr.ip())
            .collect();
        ips.shuffle(&mut rand::rng());
        Ok(ips)
    }
}
