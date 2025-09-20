# deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

## Description

**deboa** is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information.

## Features

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set base url only once, change it when needed
- request data only by specifying path
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, br)
- pluggable serialization (json, xml, msgpack)
- bora macro to easily create api clients
- cookies support
- comprehensive error handling
- runtime compatibility (tokio and smol)
- http1/2 support 

## Install

```rust
deboa = { version = "0.0.5-alpha.3", features = ["http1", "tokio-rt"] }
```

## Crate features

- tokio-rt (default)
- smol-rt
- http1 (default)
- http2

## Usage

```rust
use deboa::{Deboa, request::get};
use deboa_extras::http::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
  let client = Deboa::new();

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
    .bearer_auth("token")
    .go(client)
    .await?
    .body_as(JsonBody)?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
