# Deboa Extras

This crate provides additional features for Deboa like compression and serialization.

## Install

`cargo add deboa-extras`

## Features

- compression (gzip, deflate and brotli)
- serialization (json, xml, msgpack)

## Usage

### Decompression

```rust
use deboa::{Deboa, errors::DeboaError, interceptor::DeboaInterceptor, request::DeboaRequest};
use deboa_extras::{
    interceptor::encoding::EncodingInterceptor,
    io::brotli::BrotliDecompressor
};

let encoding_interceptor = EncodingInterceptor::register_decoders(vec![Box::new(BrotliDecompressor)]);

let mut client = Deboa::builder()
  .add_interceptor(encoding_interceptor)
  .build()?

let posts = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts/1")
  .send_client(&mut client)
  .await?;

println!("{:?}", posts.raw_body());
```

### Serialization

```rust
use deboa::{Deboa, errors::DeboaError, request::DeboaRequest};
use deboa_extras::http::serde::json::JsonBody;

let mut client = Deboa::new();

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = DeboaRequest::post("https://jsonplaceholder.typicode.com/posts/1")
  .body_as(JsonBody, data)?
  .send_with(&mut client)
  .await?;

println!("Response Status Code: {}", response.status());
```

