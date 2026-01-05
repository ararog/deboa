# Deboa Extras

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-extras)](https://crates.io/crates/deboa-extras) [![crates.io](https://img.shields.io/crates/v/deboa-extras?style=flat-square)](https://crates.io/crates/deboa-extras) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-extras) [![Documentation](https://docs.rs/deboa-extras/badge.svg)](https://docs.rs/deboa-extras/latest/deboa-extras) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa-extras) 


This crate provides additional features for Deboa like compression and serialization.

## Install

Either run from command line:

`cargo add deboa-extras`

Or add to your `Cargo.toml`:

```toml
deboa-extras = "0.0.1"
```

## Features

- `json` serialization
- `msgpack` serialization
- `xml` serialization
- `gzip` compression
- `brotli` compression
- `deflate` compression
- `websocket` support
- `sse` support

## Usage

### Decompression

```rust
use deboa::{Deboa, errors::DeboaError, interceptor::DeboaCatcher, request::DeboaRequest};
use deboa_extras::{
    interceptor::encoding::EncodingCatcher,
    io::brotli::BrotliDecompressor,
    http::serde::json::JsonBody
};

let encoding_catcher = EncodingCatcher::register_decoders(vec![Box::new(BrotliDecompressor)]);

let client = Deboa::builder()
  .catch(encoding_catcher)
  .build()?

let posts = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts/1")?
  .send_with(&client)
  .await?
  .body_as(JsonBody)?;

println!("{:?}", posts.raw_body());
```

### Serialization

```rust
use deboa::{Deboa, errors::DeboaError, request::post};
use deboa_extras::http::serde::json::JsonBody;

let client = Deboa::default();

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = post("https://jsonplaceholder.typicode.com/posts/1")?
  .body_as(JsonBody, data)?
  .send_with(client)
  .await?;

println!("Response Status Code: {}", response.status());
```

### SSE

```rust
use deboa::{Deboa, Result};
use deboa_extras::http::sse::response::{IntoEventStream};

let client = Deboa::default();

let response = client.execute("https://sse.dev/test").await?.into_event_stream();

// Poll events, until the connection is closed
// please note that this is a blocking call
while let Some(event) = response.next().await {
    println!("event: {}", event);
}

println!("Connection closed");
```

### Websockets

```rust
use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::ws::{
    io::socket::DeboaWebSocket,
    protocol::{self},
    request::WebsocketRequestBuilder,
    response::IntoWebSocket,
};

let client = Deboa::default();

let websocket = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
    .send_with(&client)
    .await?
    .into_websocket()
    .await;

while let Ok(()) = websocket.read_message().await {
    // Just keep checking messages
}
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
