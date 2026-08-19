use deboa::{
    dns::{DnsResolver, DnsResolverFuture},
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
    fn resolve(&self, host: String, port: u16) -> DnsResolverFuture {
        let future = async move {
            let hostname = format!("{}:{}", host, port);
            // `getaddrinfo` is a blocking syscall, so it goes to a thread —
            // as it does in every runtime, tokio and compio included.
            //
            // **Via the `blocking` crate rather than glommio's own pool**, and
            // that is worth explaining: `DnsResolverFuture` is
            // `Pin<Box<dyn Future + Send>>`, and glommio's `spawn_blocking`
            // returns a handle bound to its local executor, which is `!Send`.
            // compio's is a thread pool and therefore `Send`, which is why
            // `deboa-compio` can use its runtime's own. `blocking::unblock`
            // gives a `Send` future and is runtime-agnostic, so it fits the
            // trait as written. If the boxed future ever loses its `Send`
            // bound, this becomes `glommio::executor().spawn_blocking(..)` and
            // the extra thread pool goes away.
            let addrs = blocking::unblock(move || {
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
        };
        Box::pin(future)
    }
}
