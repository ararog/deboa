# deboa-smol

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-smol)](https://crates.io/crates/deboa-smol) [![crates.io](https://img.shields.io/crates/v/deboa-smol?style=flat-square)](https://crates.io/crates/deboa-smol) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-smol) [![Documentation](https://docs.rs/deboa-smol/badge.svg)](https://docs.rs/deboa-smol/latest/deboa-smol) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/ararog/deboa/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/ararog/deboa)

## Description

**deboa-smol** ("fine" portuguese slang) is a deboa implementation for smol runtime.

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
- http1/2/3 support

## Benchmark Results

As of the latest benchmark run, Deboa demonstrates competitive performance compared to Reqwest.

### Get Request

|            | `Deboa`                  | `Reqwest`                        |
|:-----------|:-------------------------|:-------------------------------- |
| **`100`**  | `46.37 ms` (✅ **1.00x**) | `48.67 ms` (✅ **1.05x slower**)  |
| **`500`**  | `46.47 ms` (✅ **1.00x**) | `47.32 ms` (✅ **1.02x slower**)  |
| **`1000`** | `46.36 ms` (✅ **1.00x**) | `47.34 ms` (✅ **1.02x slower**)  |

## Install

Either run from command line:

`cargo add deboa-smol http`

Or add to your `Cargo.toml`:

```toml
deboa-smol = { version = "0.0.9", features = ["http1"] }
http = "1.3.1"
```

## Crate features

- http1
- http2 (default)
- http3
- rust-tls (default)
- native-tls

## Usage

```rust
use deboa::{
    request::{DeboaRequest, FetchWith, get},
    Result,
};
use deboa_smol::Client;
use deboa_extras::serde::json::JsonBody;
use ::http::Method;
use macro_rules_attribute::apply;
use smol_macros::main;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[apply(main!)]
async fn main() -> Result<()> {
    let client = Client::default();

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

    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
      .send_with(&client)
      .await?
      .body_as(JsonBody)
      .await?;

    println!("posts: {:#?}", posts);
    */

    Ok(())
}
```

## Create project from template

You can create a new project from the template using `cargo generate`:

`cargo generate ararog/deboa-templates`

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
