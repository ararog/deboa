---
layout: default
title: Vamo Bora - REST Client Generator
nav_order: 4
---

# Vamo Bora

A macro for easy REST client generation on top of the Vamo REST client.

## Features

- Generate type-safe REST clients from trait definitions
- Automatic serialization/deserialization
- Path and query parameter support
- Request body handling
- Custom headers and middleware

## Installation

```toml
[dependencies]
deboa = { version = "0.0.5" }
deboa-tokio = { version = "0.0.5" }
vamo-macros = { version = "0.0.5", features = ["json"] }
```

## Basic Example

```rust
use deboa::Result;
use deboa_tokio::Client;
use vamo::Vamo;
use vamo_macros::bora;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
    api(
        get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
        get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
    )
)]
pub struct PostService;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Vamo::<Client>::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(client);

    let post = post_service.get_by_id(1).await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);
    
    assert_eq!(post.id, 1);
    Ok(())
}
```

## Features

- `json`: Enable JSON serialization/deserialization (requires `serde_json`)
- `xml`: Enable XML serialization/deserialization (requires `serde_json`)
- `msgpack`: Enable MessagePack serialization/deserialization (requires `serde_json`)


## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vamo-macros).
