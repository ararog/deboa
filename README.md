# deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

## Description

**deboa** is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information. Keep in mind API for 0.0.5 is subject to change in alpha releases. Proper deprecation will be added in the next stable release.

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
use deboa::{Deboa, errors::DeboaError, request::DeboaRequest};
use deboa_extras::http::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
  let client = Deboa::new();

  let posts: Vec<Post> = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts")?
    .header(header::CONTENT_TYPE, "application/json")
    .bearer_auth("token")
    .go(client)
    .await?
    .body_as(JsonBody)?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## Subprojects

### deboa-extras

Pluggable compression/decompression, serializers and catchers.

### deboa-macros

A crate with bora macro, for easy rest client generation.

### vamo

Nice wrapper on top of deboa for dry rest client.

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
