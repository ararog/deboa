# deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

## Description

**deboa** ("I'm ok" in portuguese) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information.

## Features

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, br)
- pluggable serialization (json, xml, msgpack)
- bora macro to easily create api clients
- cookies support
- comprehensive error handling
- runtime compatibility (tokio and smol)
- http1/2 support 
- http3 support (soon)

## Install

```rust
deboa = { version = "0.0.7", features = ["http1", "tokio-rt"] }
```

## Crate features

- tokio-rt (default)
- smol-rt
- http1 (default)
- http2
- http3 (soon)

## Usage

```rust
use deboa::{Deboa, request::get};
use deboa_extras::http::serde::json::JsonBody;

#[tokio::main]
async fn main() -> Result<()> {
  let client = Deboa::new();

  /* 
  You can also use the Fetch trait to issue requests
  
  let posts: Vec<Post> = "https://jsonplaceholder.typicode.com/posts"
    .fetch(client)
    .await?
    .body_as(JsonBody)?;    

  or use at, from (defaults to GET) and to (defaults to POST) methods:

  let posts: Vec<Post> = at("https://jsonplaceholder.typicode.com/posts", http::Method::GET)?
    .go(client)
    .await?
    .body_as(JsonBody)?;

  shifleft? Yes sir! Defaults to GET, but you can change it, same for headers.

  let request = client << "https://jsonplaceholder.typicode.com/posts";
  let posts: Vec<Post> = client.execute(request)
    .await?
    .body_as(JsonBody)?;

  or simply:

  let response = client.execute("https://jsonplaceholder.typicode.com/posts")
    .await?;
  let posts: Vec<Post> = response.body_as(JsonBody)?;
  */

  let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
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
