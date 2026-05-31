---
layout: default
title: Deboa - A Rust HTTP Client
nav_order: 1
description: "A straightforward, non-opinionated, developer-centric HTTP client library for Rust"
permalink: /
---
<div align="center">
<h1><b>Deboa</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) 
[![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) 
[![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**deboa** ("fine" portuguese slang) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

Built on top of [hyper](https://github.com/hyperium/hyper).

## Features

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, brotli)
- pluggable serialization (json, xml, msgpack, yaml, fory and cbor)
- cookies support
- urlencoded and multipart forms
- comprehensive error handling
- response streaming
- upgrade support (websocket, etc.)
- runtime compatibility (tokio and smol)
- http1/2/3 support

## Benchmark Results

As of the latest benchmark run, Deboa demonstrates competitive performance compared to Reqwest.

### Get Request

|            | `Deboa`                  | `Reqwest`                        |
|:-----------|:-------------------------|:-------------------------------- |
| **`100`**  | `46.37 ms` (✅ **1.00x**) | `48.67 ms` (✅ **1.05x slower**)  |
| **`500`**  | `46.47 ms` (✅ **1.00x**) | `47.32 ms` (✅ **1.02x slower**)  |
| **`1000`** | `46.36 ms` (✅ **1.00x**) | `47.34 ms` (✅ **1.02x slower**)  |

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.9" }
deboa-tokio = { version = "0.1.0-beta.2" }
```

Basic usage:

```rust
use deboa::{
    request::{DeboaRequest, FetchWith, get},
    Result, 
};
use deboa_tokio::Client;
use deboa_extras::http::{self, serde::json::JsonBody};

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

## Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| [deboa](./deboa) | Core HTTP client library | [![docs.rs](https://img.shields.io/docsrs/deboa/latest)](https://docs.rs/deboa) |
| [deboa-smol](./deboa-smol) | Smol runtime support for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-smol/latest)](https://docs.rs/deboa-smol) |
| [deboa-tokio](./deboa-tokio) | Tokio runtime support for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-tokio/latest)](https://docs.rs/deboa-tokio) |
| [deboa-extras](./deboa-extras) | Additional functionality and middleware | [![docs.rs](https://img.shields.io/docsrs/deboa-extras/latest)](https://docs.rs/deboa-extras) |
| [deboa-fory](./deboa-fory) | Apache Fory support for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-fory/latest)](https://docs.rs/deboa-fory) |
| [deboa-macros](./deboa-macros) | Procedural macros for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-macros/latest)](https://docs.rs/deboa-macros) |
| [vamo](./vamo) | DRY REST client wrapper | [![docs.rs](https://img.shields.io/docsrs/vamo/latest)](https://docs.rs/vamo) |
| [vamo-macros](./vamo-macros) | Macros for Vamo | [![docs.rs](https://img.shields.io/docsrs/vamo-macros/latest)](https://docs.rs/vamo-macros) |

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Deboa in your projects.

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/deboa-templates`

## Documentation

- [API Reference](https://docs.rs/deboa)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
