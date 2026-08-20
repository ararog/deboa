# Spike: glommio support for deboa

**Run 2026-08-19, against deboa `c7ce9179`. Not pushed anywhere.**

## Question

Does deboa's abstraction admit a **per-core, `!Send`** runtime, or does it only
look runtime-agnostic because tokio and smol are both `Send`-shaped?

## Answer: yes, and it took a few hours

`deboa-glommio` — copied from `deboa-smol` and ported — issues real requests on
a bare `glommio::LocalExecutor`, no tokio or smol anywhere in the process:

| | result |
|---|---|
| HTTP/1.1 plain, local MinIO | **200 OK** |
| HTTP/1.1 + TLS, `https://example.com` | **200 OK** |
| HTTP/2 + TLS (ALPN h2), `https://example.com` | **200 OK** |

## Why it works

The connection traits in `deboa/src/conn.rs` — `HttpConnection`,
`HttpConnectionPool`, `HttpConnectionDispatcher`, `ProtoConnection` — use
`impl Future` in trait position and carry **no `Send`, `Sync` or `'static`
bounds**. That is the difference between this and, say, `iceberg::Catalog`,
which is `#[async_trait]` and therefore boxes into `dyn Future + Send`.

`deboa-compio` having gone first helps: compio is also completion-based and
thread-per-core, so the abstraction had already met a runtime of this shape.

## The port, in full

Mechanical, and small:

1. `rt/stream.rs` — `SmolStream` → `GlommioStream`; glommio's `TcpStream`
   already implements `futures::io::{AsyncRead, AsyncWrite}`, so the enum and
   its two impls carried over unchanged apart from the type.
2. `rt/executor.rs` — `smol::spawn` → `glommio::spawn_local`. **The `Send`
   bound on the impl was dropped**: hyper's `Executor` trait does not require
   one, the smol and tokio bindings add it because their spawns do.
3. `client/http/http1.rs`, `http2.rs` — same one-line spawn swap.
   `smol_hyper::rt::FuturesIo` was reused as-is; it is generic over
   `futures-io` and drags no runtime.
4. `client/http/conn/stream/plain.rs` — dial through `glommio::net::TcpStream`.
5. `cert.rs` — `smol::fs::read` → `std::fs::read` (certificate loading is
   startup-time and blocking is honest there).
6. `client/dns.rs` — see below.

Dropped for the spike, not because they resist porting: websockets, HTTP/3
(quinn), native-tls.

## What deboa would need to change to take this upstream

**One real blocker, and it is small.** `deboa/src/dns.rs`:

```rust
pub type DnsResolverFuture = Pin<Box<dyn Future<Output = Result<Vec<IpAddr>>> + Send>>;
```

The `+ Send` means a per-core resolver cannot use its runtime's blocking pool —
glommio's `spawn_blocking` handle is `!Send`, as compio's presumably is. The
spike calls `getaddrinfo` inline instead, which blocks the executor for the
duration of a lookup. Acceptable for a spike; not for a merge.

Fix options, in order of preference:

1. Drop `+ Send` from the boxed future. Costs the other bindings nothing — a
   `Send` future satisfies a `?Send` box.
2. Make the future an associated type on `DnsResolver`, so each binding names
   its own.

Everything else compiled without touching a line of `deboa` core.

## Recommendation

Worth pursuing. Contributing `deboa-glommio` upstream is cheaper than finishing
our own client (see `docs/http-client-roadmap.md`) and gets redirects,
timeouts, cookies, caching, HTTP/3 and a maintained release cadence for free.

Caveats to weigh, unchanged from the assessment:

- deboa is `0.1.0-beta.23`, README warns of major API changes. Pin by rev, as
  we do for glommio and barnabas.
- deboa is client-only. `slipstream-http` still owns the hyper **server** and
  the `object_store` `HttpConnector` facade either way.
- `Client::default()` builds a `!Send` client; the `object_store` facade would
  still be a channel in front of a per-core worker, exactly as it is now.

## Reproducing

```sh
cd ~/projects/deboa      # this clone, outside the slipstream repo
cargo run -p deboa-glommio --example spike --no-default-features \
  --features http1,rust-tls,default-rustls-provider,default-rustls-verifier \
  -- http://127.0.0.1:9000/minio/health/live
H2=1 cargo run -p deboa-glommio --example spike -- https://example.com
```
