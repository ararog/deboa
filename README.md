# deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

## Description

**deboa** is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

Built using [hyper](https://github.com/hyperium/hyper).

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information. Keep in mind API for prior to 0.1.0 is subject to change. Proper deprecation will be added in the next stable release.

## Install

```rust
deboa = { version = "0.0.7", features = ["http1", "tokio-rt"] }
```

## Runtimes

- [tokio](https://github.com/tokio-rs/tokio)
- [smol](https://github.com/smol-rs/smol)

## Crate features

- tokio-rt (default)
- smol-rt
- http1 (default)
- http2

## Usage

```rust
use deboa::{Deboa, errors::DeboaError, request::get};
use deboa_extras::http::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new Deboa instance, set timeouts, catches and protocol.
  let client = Deboa::new();

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
    .header(header::CONTENT_TYPE, "application/json")
    .go(client)
    .await?
    .body_as(JsonBody)
    .await?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## Subprojects

### deboa-bora

A crate with bora macro, for easy rest client generation.

### deboa-extras

Pluggable compression/decompression, serializers and catchers.

### deboa-macros

A crate with set of convenience macros.

### deboa-tests

A crate with testing utilities.

### examples

Examples of how to use deboa.

### vamo

Nice wrapper on top of deboa for dry rest client.

### vamo-macros

Vamo macros is a collection of macros to make possible
use structs as resources to be sent over vamo as client.

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
