<div align="center">

<img src="https://raw.githubusercontent.com/ararog/deboa/refs/heads/develop/other_deboa_128.png" alt="deboa" width="128" height="128">

# Deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![codecov](https://codecov.io/gh/ararog/deboa/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/deboa) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

</div>

## Description

**deboa** ("fine" portuguese slang) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

Built using [hyper](https://github.com/hyperium/hyper).

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information. Keep in mind API for prior to 0.1.0 is subject to change. Proper deprecation will be added in the next stable release.

## Install

```rust
deboa = { version = "0.0.9", features = ["http1", "http2", "tokio-rt"] }
```

## Runtimes

- [tokio](https://github.com/tokio-rs/tokio)
- [smol](https://github.com/smol-rs/smol)

## Crate features

- http1
- http2 (default)
- http3
- rust-tls
- native-tls

## Usage

```rust
use deboa::{Client, errors::DeboaError, request::get, Result};
use deboa_extras::http::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<()> {
  // Create a new Client instance, set timeouts, catches and protocol.
  let client = Client::new();

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
    .header(header::CONTENT_TYPE, "application/json")
    .send_with(&client)
    .await?
    .body_as(JsonBody)
    .await?;

  println!("posts: {:#?}", posts);

  Ok(())
}
```

## Subprojects

### [deboa](https://github.com/ararog/deboa/tree/develop/deboa)

The core create of http client.

### deboa-bora (removed)

A crate with bora macro, for easy rest client generation. Bora macro is now part of vamo-macros.

### [deboa-extras](https://github.com/ararog/deboa/tree/develop/deboa-extras)

Pluggable compression/decompression, serializers, sse, websockets and catchers.
All of them are optional. This is the place to contribute with your own pluggable features.

### [deboa-macros](https://github.com/ararog/deboa/tree/develop/deboa-macros)

A crate with collection of convenience macros for deboa. It is close equivalent to
apisauce for axios, where one macro does it all, from request to response.
It used to be the home of bora macro, which has been moved to vamo-macros crate.

### [deboa-smol](https://github.com/ararog/deboa/tree/develop/deboa-smol)

Deboa implementation for smol runtime.

### [deboa-tokio](https://github.com/ararog/deboa/tree/develop/deboa-tokio)

Deboa implmentation for tokio runtime.

### [vamo](https://github.com/ararog/deboa/tree/develop/vamo)

Nice wrapper on top of deboa for dry rest client. Set base url once
and use it for all requests.

### [vamo-macros](https://github.com/ararog/deboa/tree/develop/vamo-macros)

Vamo macros is a collection of macros to make possible use structs as resources to be sent over vamo as client.
It is also the new home of bora macro.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
