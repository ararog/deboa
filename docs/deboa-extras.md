---
layout: default
title: Deboa Extras - serializers, compression, websockets and sse support for Deboa
nav_order: 3
---

# Deboa Extras

Additional functionality for the Deboa HTTP client, including serializers, compression, websockets and sse support.

## Features

- **Serialization/Deserialization**
  - JSON
  - MessagePack
  - XML

- **Compression**
  - Gzip
  - Brotli
  - Deflate

- **WebSockets**

- **Server-Sent Events**

## Installation

```toml
[dependencies]
deboa-extras = { version = "0.0.7", features = ["json", "websocket", "sse"] }
```

## Serialization

### JSON

```rust
use deboa_extras::http::serde::json::JsonBody;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
}

// Serialize request body
let user = User { id: 1, name: "John Doe".to_string() };
let request = deboa::post("https://api.example.com/users")
    .body_as(JsonBody, &user)?;

// Deserialize response
let response: User = request
    .send_with(&client)
    .await?
    .body_as(JsonBody)?;
```

## Features

- `json`: JSON serialization/deserialization (requires `serde_json`)
- `msgpack`: MessagePack serialization
- `xml`: XML serialization

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-extras).
