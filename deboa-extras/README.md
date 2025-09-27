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
  .go(client)
  .await?
  .body_as(JsonBody)?;

println!("{:?}", posts.raw_body());
```

### Serialization

```rust
use deboa::{Deboa, errors::DeboaError, request::post};
use deboa_extras::http::serde::json::JsonBody;

let client = Deboa::new();

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = post("https://jsonplaceholder.typicode.com/posts/1")?
  .body_as(JsonBody, data)?
  .go(client)
  .await?;

println!("Response Status Code: {}", response.status());
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
