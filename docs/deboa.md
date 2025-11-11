---
layout: default
title: Deboa - Core HTTP Client
nav_order: 2
---

# Deboa Core

The core HTTP client library for Rust, providing a simple yet powerful interface for making HTTP requests.

## Features

- Async/await support
- HTTP/1.1 and HTTP/2 support
- Extensible middleware system
- Request and response builders
- Timeout and retry configuration
- Cookie handling
- Automatic request/response serialization/deserialization

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
deboa = { version = "0.0.7", features = ["http1", "tokio-rt"] }
```

## Basic Usage

```rust
use deboa::{Deboa, request::get};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Deboa::new();
    
    // Make a GET request
    let response = get("https://httpbin.org/get")
        .go(&client)
        .await?;
        
    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);
    
    Ok(())
}
```

## Making Requests

### GET Request

```rust
let response = deboa::get("https://api.example.com/data")
    .header("Accept", "application/json")
    .go(&client)
    .await?;
```

### POST Request with JSON

```rust
use serde_json::json;

let data = json!({ "name": "John Doe", "age": 30 });

let response = deboa::post("https://api.example.com/users")
    .json(&data)?
    .go(&client)
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
    .go(&client)
    .await?
    .body_as_json()?;

// Get response as text
let text = response.text().await?;

// Get response as bytes
let bytes = response.bytes().await?;
```

## Middleware

Deoba supports middleware for request/response processing:

```rust
use deboa::middleware::{Middleware, Next};

struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(&self, req: deboa::Request, next: Next<'_>) -> Result<deboa::Response, deboa::Error> {
        println!("Sending request to {} {}", req.method(), req.uri());
        let start = std::time::Instant::now();
        let res = next.run(req).await?;
        let duration = start.elapsed();
        println!("Received response in {:?} - {}", duration, res.status());
        Ok(res)
    }
}

// Create a client with middleware
let client = deboa::Deboa::builder()
    .with(LoggingMiddleware)
    .build();
```

## Error Handling

Deoba provides comprehensive error handling through the `deboa::Error` type:

```rust
match deboa::get("https://api.example.com/data").go(&client).await {
    Ok(response) => {
        // Handle successful response
    }
    Err(deboa::Error::Http(e)) => {
        // Handle HTTP errors
    }
    Err(deboa::Error::Json(e)) => {
        // Handle JSON parsing errors
    }
    Err(e) => {
        // Handle other errors
    }
}
```

## Features

- `http1`: Enable HTTP/1 support (enabled by default)
- `http2`: Enable HTTP/2 support
- `tokio-rt`: Use Tokio as the async runtime (enabled by default)
- `smol-rt`: Use smol as the async runtime

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa).
