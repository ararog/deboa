---
layout: default
title: Deboa - Core HTTP Client
nav_order: 2
---

## Deboa Core

The core HTTP client library for Rust, providing a simple yet powerful interface for making HTTP requests.

With Deboa, you can:

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- compression (gzip, deflate, brotli)
- pluggable hooks (interceptors)
- pluggable serialization (json, xml, msgpack, yaml, fory and cbor)
- cookies support
- urlencoded and multipart forms
- comprehensive error handling
- response streaming
- upgrade support (websocket, etc.)
- runtime compatibility (tokio, smol and compio)
- http 1/2/3 support (via runtime crates)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.9" }
```

## Basic Usage

```rust
use deboa::{request::get, Result};
use deboa_tokio::Client;

#[tokio::main]
async fn main() -> Result<(), Result> {
    let client = Client::new();

    // Make a GET request
    let response = get("https://httpbin.org/get")
        .send_with(&client)
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
```

## Making Requests

### GET Request

```rust
use deboa::request::get;

let response = get("https://api.example.com/data")
    .header("Accept", "application/json")
    .send_with(&client)
    .await?;

// OR

let response = "GET".from_url("https://api.example.com/data")
    .header("Accept", "application/json")
    .send_with(&client)
    .await?;
```

### POST Request with JSON

```rust
use deboa_extras::serde::json::JsonBody;
use serde_json::json;

let data = json!({ "name": "John Doe", "age": 30 });

let response = deboa::post("https://api.example.com/users")
    .body_as(JsonBody, &data)?
    .send_with(&client)
    .await?;
```

### Handling Responses

```rust
#[derive(serde::Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Parse JSON response into a struct
let user: User = deboa::get("https://api.example.com/users/1")
    .send_with(&client)
    .await?
    .body_as(JsonBody)?;

// Get response as text
let text = response.text().await?;

// Get response as bytes
let bytes = response.bytes().await?;
```

## Hooks (Middleware)

Deboa supports middleware for request/response processing:

```rust
use deboa::{Result, catcher::DeboaCatcher, request::DeboaRequest, response::DeboaResponse};

use tackle::{Chain, Hook};

struct PrintRequestHook<H> {
    inner: H,
}

impl<H> Hook<DeboaRequest, DeboaResponse> for PrintRequestHook<H>
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    type Result = Result<DeboaResponse>;
    type Error = DeboaError;

    async fn call(&self, request: DeboaRequest) -> Self::Result {
        println!("Request: {:?}", request);
        let res = self.inner
            .call(request)
            .await
        println!("Response: {:?}", response);
        res
    }
}

struct PrintRequest;

impl<H> Chain<H, DeboaError, DeboaRequest, DeboaResponse> for PrintRequest
where
    H: Hook<DeboaRequest, DeboaResponse, Result = Result<DeboaResponse>>,
{
    type Hook = PrintRequestHook<H>;

    fn chain(&self, hook: H) -> Self::Hook {
        PrintRequestHook { inner: hook }
    }
}

// Create a client with middleware
let client = deboa::Client::default()
    .hook(PrintRequest);
```

## Error Handling

Deboa provides comprehensive error handling through the `deboa::errors::DeboaError` type:

```rust
match deboa::get("https://api.example.com/data").send_with(&client).await {
    Ok(response) => {
        // Handle successful response
    }
    Err(DeboaError::Connection(e)) => {
        // Handle connection errors
        eprintln!("Connection failed: {}", e);
    },
    Err(DeboaError::Request(e)) => {
        // Handle request errors
        eprintln!("Request failed: {}", e);
    },
    Err(e) => {
        // Handle other errors
    }
}
```

## Examples

See the [examples](../examples.md) for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa).
