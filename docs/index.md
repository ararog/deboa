---
layout: default
title: Deboa - A Rust HTTP Client
nav_order: 1
description: "A straightforward, non-opinionated, developer-centric HTTP client library for Rust"
permalink: /
---

# Deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) 
[![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) 
[![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A straightforward, non-opinionated, developer-centric HTTP client library for Rust. Built on top of [hyper](https://github.com/hyperium/hyper).

## Features

- **Async/Await** - Built with Rust's async/await syntax
- **Flexible** - Supports both HTTP/1.1 and HTTP/2
- **Extensible** - Middleware support for custom functionality
- **Runtime Agnostic** - Works with tokio and smol runtimes
- **Type-safe** - Strong typing for requests and responses

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.7", features = ["http1", "tokio-rt"] }
```

Basic usage:

```rust
use deboa::{Deboa, request::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct Post {
    id: u64,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Deboa::new();
    
    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")
        .go(&client)
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
| [deboa-bora](./deboa-bora) | Macro for easy REST client generation | [![docs.rs](https://img.shields.io/docsrs/deboa-bora/latest)](https://docs.rs/deboa-bora) |
| [deboa-macros](./deboa-macros) | Procedural macros for Deboa | [![docs.rs](https://img.shields.io/docsrs/deboa-macros/latest)](https://docs.rs/deboa-macros) |
| [vamo](./vamo) | DRY REST client wrapper | [![docs.rs](https://img.shields.io/docsrs/vamo/latest)](https://docs.rs/vamo) |
| [vamo-macros](./vamo-macros) | Macros for Vamo | [![docs.rs](https://img.shields.io/docsrs/vamo-macros/latest)](https://docs.rs/vamo-macros) |

## Examples

Check out the [examples](./examples) directory for complete examples of how to use Deboa in your projects.

## Documentation

- [API Reference](https://docs.rs/deboa)
- [Migration Guide](./MIGRATION_GUIDE.md)
- [Contributing Guide](./CONTRIBUTING.md)

## License

This project is licensed under the [MIT License](./LICENSE.md).

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
