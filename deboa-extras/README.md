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
use deboa::{Deboa, errors::DeboaError};
use deboa_extras::io::gzip::GzipDecompressor;

let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;

api.accept_encoding(vec![Box::new(GzipDecompressor)]);

let posts = api.get("posts/1").await?;

println!("{:?}", posts.raw_body());
```

### Serialization

```rust
use deboa::{Deboa, errors::DeboaError};
use deboa_extras::http::serde::json::JsonBody;

let mut api = Deboa::new("https://jsonplaceholder.typicode.com")?;

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = api.set_body_as(JsonBody, data)?.post("posts/1").await?;

println!("Response Status Code: {}", response.status());
```

