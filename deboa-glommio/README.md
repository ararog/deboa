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

## The glommio dependency

This crate depends on [`glommio-ng`](https://crates.io/crates/glommio-ng),
renamed back to `glommio` in the manifest so the source reads normally:

```toml
glommio = { package = "glommio-ng", version = "0.10" }
```

The canonical [`glommio`](https://crates.io/crates/glommio) crate is stuck at
0.9.0 (March 2024), and its vendored liburing no longer compiles against
current kernel headers — it fails with `invalid application of 'sizeof' to
incomplete type 'struct open_how'` on any recent glibc. `glommio-ng` is a
republish of the community fork at
[github.com/glommio/glommio](https://github.com/glommio/glommio), which keeps
io_uring and its dependencies current.

**They are different crates and their types do not unify.** A library compiled
against `glommio-ng` cannot accept an executor or socket from canonical
`glommio`, and vice versa. If 0.10 is ever published under the canonical name
this dependency becomes a one-line change and `glommio-ng` gets deprecated.

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
puts it. This binding uses glommio's own `spawn_blocking`, so the work stays
inside the runtime that owns the executor.

That is possible because `DnsResolver::resolve` returns `impl Future` rather
than a boxed `dyn Future + Send`. The `Send` bound was more than a resolver can
promise on every runtime: compio's `spawn_blocking` returns a `Send` future and
satisfied it, glommio's handle is bound to its local executor and did not, so
this binding originally had to pull in a second thread pool alongside glommio's
to work around a bound it could not meet.
