---
layout: default
title: Deboa Extras - Additional Functionality
nav_order: 3
---

# Deboa Extras

Additional functionality for the Deboa HTTP client, including serializers, middleware, and utilities.

## Features

- **Serialization/Deserialization**
  - JSON
  - URL-encoded forms
  - Multipart forms
  - MessagePack
  - XML

- **Middleware**
  - Logging
  - Retry
  - Rate limiting
  - Authentication
  - Caching

- **Utilities**
  - Request/response interceptors
  - Mock server for testing
  - Cookie management

## Installation

```toml
[dependencies]
deboa-extras = { version = "0.0.7", features = ["json", "middleware-logger"] }
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
    .body(JsonBody::new(&user))?;

// Deserialize response
let response: User = request
    .send_with(&client)
    .await?
    .body_as(JsonBody)?;
```

## Middleware

### Logging

```rust
use deboa_extras::middleware::logger::Logger;

let client = deboa::Deboa::builder()
    .with(Logger::default())
    .build();
```

### Retry

```rust
use deboa_extras::middleware::retry::{Retry, RetryPolicy};
use std::time::Duration;

let retry_policy = RetryPolicy::new()
    .with_max_retries(3)
    .with_backoff(Duration::from_millis(100));

let client = deboa::Deboa::builder()
    .with(Retry::new(retry_policy))
    .build();
```

### Rate Limiting

```rust
use deboa_extras::middleware::rate_limit::{RateLimit, RateLimiter};

// Allow 10 requests per minute per client
let rate_limiter = RateLimiter::per_minute(10);
let client = deboa::Deboa::builder()
    .with(RateLimit::new(rate_limiter))
    .build();
```

## Authentication

### Basic Auth

```rust
use deboa_extras::auth::BasicAuth;

let client = deboa::Deboa::builder()
    .with(BasicAuth::new("username", "password"))
    .build();
```

### Bearer Token

```rust
use deboa_extras::auth::BearerToken;

let client = deboa::Deboa::builder()
    .with(BearerToken::new("your-token-here"))
    .build();
```

## Features

- `json`: JSON serialization/deserialization (requires `serde_json`)
- `form`: URL-encoded form serialization (requires `serde_urlencoded`)
- `multipart`: Multipart form data support
- `msgpack`: MessagePack serialization
- `xml`: XML serialization
- `middleware-logger`: Request/response logging
- `middleware-retry`: Automatic request retry
- `middleware-rate-limit`: Rate limiting
- `middleware-cache`: Response caching
- `testing`: Testing utilities and mock server

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-extras).
