# deboa

[![Crates.io downloads](https://img.shields.io/crates/d/deboa)](https://crates.io/crates/deboa) [![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/deboa/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/deboa)

## Description

**deboa** ("fine" portuguese slang) is a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Attention

This release has a major api change. Please check the [migration guide](https://github.com/ararog/deboa/blob/main/MIGRATION_GUIDE.md) for more information.

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
- http3 support (soon)

## Benchmark Results

### Get Request

|            | `Deboa`                  | `Reqwest`                        |
|:-----------|:-------------------------|:-------------------------------- |
| **`100`**  | `50.53 ms` (✅ **1.00x**) | `70.92 ms` (❌ *1.40x slower*)    |
| **`500`**  | `46.44 ms` (✅ **1.00x**) | `71.23 ms` (❌ *1.53x slower*)    |
| **`1000`** | `96.79 ms` (✅ **1.00x**) | `99.82 ms` (✅ **1.03x slower**)  |


## Install

Either run from command line:

`cargo add deboa http`

Or add to your `Cargo.toml`:

```toml
deboa = { version = "0.0.9", features = ["http1", "tokio-rt"] }
http = "1.3.1"
```

## Crate features

- tokio-rt (default)
- smol-rt
- http1 (default)
- http2
- http3 (soon)

## Usage

```rust
use deboa::{
    Client, Result, request::{DeboaRequest, FetchWith, get}
};
use deboa_extras::http::{self, serde::json::JsonBody};

use ::http::Method;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::new();

    /*

    // You can also use the Fetch trait to issue requests

    let posts: Vec<Post> = "https://jsonplaceholder.typicode.com/posts"
      .fetch_with(client)
      .await?
      .body_as(JsonBody)
      .await?;

    // or use at, from (defaults to GET) and to (defaults to POST) methods:

    let posts: Vec<Post> = DeboaRequest::at("https://jsonplaceholder.typicode.com/posts", Method::GET)?
      .send_with(client)
      .await?
      .body_as(JsonBody)
      .await?;

    // shifleft? Yes sir! Defaults to GET, but you can change it, same for headers.
 
    let request = &client << "https://jsonplaceholder.typicode.com/posts";
    let posts: Vec<Post> = client.execute(request)
      .await?
      .body_as(JsonBody)
      .await?;

    // or simply:

    let posts: Vec<Post> = client
      .execute("https://jsonplaceholder.typicode.com/posts")
      .await?
      .body_as(JsonBody)
      .await?;

    // you can also post a json body

    let body = serde_json::json!({
      "id": 100,
      "title": "Some title",
      "body": "Some body"
    });

    let request = post("https://jsonplaceholder.typicode.com/posts")?
      .header(header::CONTENT_TYPE, "application/json")
      .body_as(JsonBody, body)?;
    let response = request.send_with(&mut client).await?;
    assert_eq!(response.status(), 201);

    */

    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
      .send_with(client)
      .await?
      .body_as(JsonBody)
      .await?;

    println!("posts: {:#?}", posts);

    Ok(())
}
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
