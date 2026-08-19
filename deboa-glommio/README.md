# deboa-glommio

## Description

**deboa-glommio** is a deboa implementation for the [glommio](https://github.com/DataDog/glommio)
runtime: thread-per-core, `io_uring`-backed, and single-threaded by
construction. Its sockets and tasks are `!Send`, so a client built on it never
leaves the core that opened its connections.

## Status

Supported: HTTP/1.1 and HTTP/2, plain and over TLS (`rust-tls`), connection
pooling, and the shared deboa request/response surface.

Not supported yet, and each fails with a clear `compile_error!` rather than a
puzzle:

- **`native-tls`** — `async-native-tls` selects its runtime by feature
  (`runtime-tokio` / `runtime-smol`) and has no glommio binding.
- **`http3`** — needs a `quinn::Runtime` implementation for glommio.
- **`websockets`** — not ported yet.

## Usage

```rust
use deboa::request::{get, FetchWith};
use deboa_glommio::Client;

fn main() {
    glommio::LocalExecutorBuilder::default()
        .make()
        .unwrap()
        .run(async {
            let client = Client::default();
            let response = get("https://example.com")
                .unwrap()
                .send_with(&client)
                .await
                .unwrap();
            println!("{}", response.status());
        });
}
```

There is no `#[glommio::main]`-style attribute in the runtime, so the executor
is built explicitly. `LocalExecutorPoolBuilder` runs one per core when you want
the whole machine.

## A note on DNS

`getaddrinfo` is blocking in every runtime; the difference is where each one
puts it. This binding uses the runtime-agnostic
[`blocking`](https://crates.io/crates/blocking) thread pool rather than
glommio's own `spawn_blocking`, because `deboa::dns::DnsResolverFuture` is
`Pin<Box<dyn Future + Send>>` and glommio's blocking handle is bound to its
local executor, hence `!Send`. compio's is a thread pool and therefore `Send`,
which is why `deboa-compio` can use its runtime's own.

Relaxing that boxed future to `?Send` would let this binding use glommio's pool
and drop the extra threads. Nothing else in deboa needed changing.
