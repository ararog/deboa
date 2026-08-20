use deboa::{
    dns::DnsResolver,
    errors::{DeboaError::Dns, DnsError},
};
use rand::seq::SliceRandom;
use std::net::IpAddr;

#[derive(Default, Clone)]
/// Default DNS resolver: `getaddrinfo` on glommio's blocking pool.
///
/// glommio has no async resolver — DNS is a blocking syscall, which is what
/// tokio and smol also do internally, just behind their own pools.
pub struct DefaultDnsResolver;

impl DnsResolver for DefaultDnsResolver {
    async fn resolve(&self, host: String, port: u16) -> deboa::Result<Vec<IpAddr>> {
        let hostname = format!("{}:{}", host, port);
        // `getaddrinfo` is a blocking syscall, so it goes to a thread — as it
        // does in every runtime. glommio's own blocking pool can be used now
        // that the trait no longer demands a `Send` future: the handle it
        // returns is bound to this executor, which is exactly what a
        // thread-per-core runtime wants and what the boxed `dyn Future + Send`
        // made impossible.
        let addrs = glommio::executor()
            .spawn_blocking(move || {
                std::net::ToSocketAddrs::to_socket_addrs(&hostname[..])
                    .map(|it| it.collect::<Vec<_>>())
            })
            .await;
        if let Err(e) = addrs {
            return Err(Dns(DnsError::Resolve { host, message: e.to_string() }));
        };

        let mut ips: Vec<IpAddr> = addrs
            .unwrap()
            .into_iter()
            .map(|addr| addr.ip())
            .collect();
        ips.shuffle(&mut rand::rng());
        Ok(ips)
    }
}
