---
layout: default
title: Deboa - A Rust HTTP Client
nav_order: 1
description: "A straightforward, non-opinionated, developer-centric HTTP client library for Rust"
permalink: /
---

<div style="text-align: center">
  <img src="./other_deboa_128.png" alt="deboa" width="128" height="128">

  <h1><b>Deboa</b></h1>
</div>

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) 
[![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) 
[![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A straightforward, non-opinionated, developer-centric HTTP client library for Rust. Built on top of [hyper](https://github.com/hyperium/hyper).

## Features

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, br)
- pluggable serialization (json, xml, msgpack)
- cookies support
- urlencoded and multipart forms
- comprehensive error handling
- response streaming
- upgrade support (websocket, etc.)
- runtime compatibility (tokio and smol)
- http1/2 support 
- http3 support (planned)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.1.0", features = ["http1", "tokio-rt"] }
```

Basic usage:

```rust
use deboa::{Client, request::get, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    id: u64,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    
    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")
        .send_with(&client)
        .await?
        .body_as_json()?;
    
    println!("First post: {}", posts[0].title);
    Ok(())
}
```

## Crates

| Crate | Description | Documentation |
|-------|-------------|---------------|
| [deboa](./deboa) | Core HTTP client library | [![docs.rs](https://img.shields.io/docsrs/deboa/latest)](https://docs.rs/deboa) |
| [deboa-extras](./deboa-extras) | Additional functionality and middleware | [![docs.rs](https://img.shields.io/docsrs/deboa-extras/latest)](https://docs.rs/deboa-extras) |
| [deboa-macros](./deboa-macros) | Procedural macros for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-macros/latest)](https://docs.rs/deboa-macros) |
| [vamo](./vamo) | DRY REST client wrapper | [![docs.rs](https://img.shields.io/docsrs/vamo/latest)](https://docs.rs/vamo) |
| [vamo-macros](./vamo-macros) | Macros for Vamo | [![docs.rs](https://img.shields.io/docsrs/vamo-macros/latest)](https://docs.rs/vamo-macros) |

## Examples

Check out the [examples](./examples.md) for complete examples of how to use Deboa in your projects.

## Documentation

- [API Reference](https://docs.rs/deboa)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
