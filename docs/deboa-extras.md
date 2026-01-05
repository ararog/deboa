---
layout: default
title: Deboa Extras - serializers, compression, websockets and sse support for Deboa
nav_order: 3
---

# Deboa Extras

Additional functionality for the Deboa HTTP client, including serializers, compression, websockets and sse support.

## Installation

```toml
[dependencies]
deboa-extras = { version = "0.0.5", features = ["json", "websocket", "sse"] }
```

## Features

- `json` (default)
- `msgpack` (default)
- `xml` (default)
- `yaml`
- `flex`
- `cbor`
- `gzip` (default)
- `brotli`
- `deflate`
- `sse` (default)
- `utils` (default)
- `websocket` (default)

## Examples

### JSON Serialization/Deserialization

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

### SSE

```rust
use deboa::{Client, Result};
use deboa_extras::http::sse::response::{IntoEventStream};

let client = Client::new();

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
use deboa::{Client, Result, request::DeboaRequestBuilder};
use deboa_extras::ws::{
    io::socket::DeboaWebSocket,
    protocol::{self},
    request::WebsocketRequestBuilder,
    response::IntoWebSocket,
};

let client = Client::new();

let websocket = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
    .send_with(&client)
    .await?
    .into_websocket()
    .await;

while let Ok(()) = websocket.read_message().await {
    // Just keep checking messages
}
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-extras).
