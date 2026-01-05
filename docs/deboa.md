---
layout: default
title: Deboa - Core HTTP Client
nav_order: 2
---

# Deboa Core

The core HTTP client library for Rust, providing a simple yet powerful interface for making HTTP requests.

With Deboa, you can:

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, brotli)
- pluggable serialization (json, xml, msgpack, yaml, fory and cbor)
- cookies support
- urlencoded and multipart forms
- comprehensive error handling
- response streaming
- upgrade support (websocket, etc.)
- runtime compatibility (tokio and smol)
- http1/2/3 support

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.9", features = ["http1", "http2", "tokio-rt", "tokio-rust-tls"] }
```

## Features

- `http1`: Enable HTTP/1 support (enabled by default)
- `http2`: Enable HTTP/2 support (enabled by default)
- `http3`: Enable HTTP/3 support (tokio only)
- `tokio-rt`: Use Tokio as the async runtime (enabled by default)
- `smol-rt`: Use smol as the async runtime
- `tokio-rust-tls`: Use tokio-rust-tls as the TLS implementation (enabled by default)
- `tokio-native-tls`: Use tokio-native-tls as the TLS implementation
- `smol-rust-tls`: Use smol-rust-tls as the TLS implementation
- `smol-native-tls`: Use smol-native-tls as the TLS implementation

## Basic Usage

```rust
use deboa::{Client, request::get, Result};

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
use deboa_extras::http::serde::json::JsonBody;
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

## Catchers (Middleware)

Deboa supports middleware for request/response processing:

```rust
use deboa::{Result, catcher::DeboaCatcher, request::DeboaRequest, response::DeboaResponse};

struct TestMonitor;

#[deboa::async_trait]
impl DeboaCatcher for TestMonitor {
    async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
        println!("Request: {:?}", request.url());
        Ok(None)
    }

    async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
        println!("Response: {:?}", response.status());
        Ok(())
    }
}

// Create a client with middleware
let client = deboa::Client::builder()
    .catch(TestMonitor)
    .build();
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
