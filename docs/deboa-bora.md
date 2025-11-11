---
layout: default
title: Deboa Bora - REST Client Generator
nav_order: 4
---

# Deboa Bora

A macro for easy REST client generation on top of the Deboa HTTP client.

## Features

- Generate type-safe REST clients from trait definitions
- Automatic serialization/deserialization
- Path and query parameter support
- Request body handling
- Custom headers and middleware

## Installation

```toml
[dependencies]
deboa-bora = { version = "0.1.0", features = ["json"] }
```

## Basic Example

```rust
use deboa_bora::bora;
use serde::{Deserialize, Serialize};

// Define your API trait
#[bora(base_url = "https://api.example.com/v1")]
trait MyApi {
    // GET /users/{id}
    #[get("/users/{id}")]
    async fn get_user(&self, id: u64) -> Result<User, Error>;
    
    // POST /users
    #[post("/users")]
    async fn create_user(&self, user: CreateUser) -> Result<User, Error>;
}

// Define your data structures
#[derive(Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Serialize)]
struct CreateUser {
    name: String,
    email: String,
}

// Use the generated client
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MyApiClient::new();
    
    // Create a new user
    let new_user = CreateUser {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    
    let user = client.create_user(new_user).await?;
    println!("Created user: {:?}", user);
    
    // Fetch a user
    let fetched_user = client.get_user(user.id).await?;
    println!("Fetched user: {:?}", fetched_user);
    
    Ok(())
}
```

## Advanced Features

### Path and Query Parameters

```rust
#[bora(base_url = "https://api.example.com")]
trait UserApi {
    // Path parameters in URL
    #[get("/users/{id}")]
    async fn get_user(&self, id: u64) -> Result<User, Error>;
    
    // Query parameters from method parameters
    #[get("/users")]
    async fn list_users(&self, page: Option<u32>, limit: Option<u32>) -> Result<Vec<User>, Error>;
}
```

### Request and Response Headers

```rust
#[bora(base_url = "https://api.example.com")]
trait SecureApi {
    // Set request headers
    #[post("/data")]
    #[header("X-API-Key: {api_key}")]
    async fn post_data(&self, api_key: String, data: Data) -> Result<(), Error>;
    
    // Read response headers
    #[get("/data/{id}")]
    #[response_header("ETag")]
    async fn get_data(&self, id: String) -> Result<(Data, String), Error>;
}
```

### Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    Http(#[from] deboa::Error),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Not found")]
    NotFound,
}

// Implement From<deboa::Error> for your error type
impl From<deboa::Error> for ApiError {
    fn from(err: deboa::Error) -> Self {
        ApiError::Http(err)
    }
}

#[bora(base_url = "https://api.example.com")]
trait UserApi {
    // Use your custom error type
    #[get("/users/{id}")]
    async fn get_user(&self, id: u64) -> Result<User, ApiError>;
}
```

## Features

- `json`: Enable JSON serialization/deserialization (requires `serde_json`)
- `form`: Enable URL-encoded form serialization (requires `serde_urlencoded`)
- `multipart`: Enable multipart form data support

## Customization

### Custom Client Configuration

```rust
use deboa::Deboa;

let client = Deboa::builder()
    .timeout(std::time::Duration::from_secs(30))
    .build();

let api = MyApiClient::with_client(client);
```

### Middleware

```rust
use deboa::middleware::Middleware;

struct AuthMiddleware {
    token: String,
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, req: deboa::request::Request, next: deboa::middleware::Next<'_>) -> Result<deboa::response::Response, deboa::Error> {
        let req = req.header("Authorization", format!("Bearer {}", self.token));
        next.run(req).await
    }
}

let client = Deboa::builder()
    .with(AuthMiddleware { token: "my-secret-token".to_string() })
    .build();

let api = MyApiClient::with_client(client);
```

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-bora).
