---
layout: default
title: Deboa Smol - HTTP Client for Smol
nav_order: 4
---

## Deboa Smol

The HTTP client library for Rust using Smol, providing a simple yet powerful interface for making HTTP requests.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.9" }
deboa-smol = { version = "0.0.9", features = ["http1", "http2", "rust-tls"] }
```

## Features

- `http1`: HTTP/1 support
- `http2`: HTTP/2 support (enabled by default)
- `http3`: HTTP/3 support
- `rust-tls`: rust-tls implementation (enabled by default)
- `native-tls`: native-tls implementation

## Basic Usage

```rust
use deboa::{request::get, Result};
use deboa_smol::Client;

#[tokio::main]
async fn main() -> Result<()> {
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
use deboa_smol::Client;

let client = Client::new();

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
use deboa::{request::post, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_smol::Client;
use serde_json::json;

let client = Client::new();
let data = json!({ "name": "John Doe", "age": 30 });
let response = post("https://api.example.com/users")
    .body_as(JsonBody, &data)?
    .send_with(&client)
    .await?;
```

### Handling Responses

```rust
use deboa::{request::get, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_smol::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Parse JSON response into a struct
let user: User = get("https://api.example.com/users/1")
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
let client = deboa_smol::Client::builder()
    .catch(TestMonitor)
    .build();
```

## Error Handling

Deboa provides comprehensive error handling through the `deboa::errors::DeboaError` type:

```rust
match deboa_smol::get("https://api.example.com/data").send_with(&client).await {
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
