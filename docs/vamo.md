---
layout: default
title: Vamo - DRY REST Client
nav_order: 6
---

# Vamo

A nice wrapper on top of deboa for creating DRY REST clients.

## Features

- Type-safe REST client generation
- Resource-based API design
- Automatic serialization/deserialization
- Middleware support
- Pagination helpers
- Request/response interceptors

## Installation

```toml
[dependencies]
vamo = { version = "0.1.0", features = ["json"] }
```

## Quick Start

```rust
use serde::{Deserialize, Serialize};
use vamo::prelude::*;

// Define your resource
#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/users")]
struct User {
    id: Option<u64>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = vamo::Client::new("https://api.example.com");
    
    // Create a new user
    let mut user = User {
        id: None,
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };
    
    // Save the user
    user = user.create(&client).await?;
    println!("Created user: {:?}", user);
    
    // Fetch all users
    let users = User::list(&client).await?;
    println!("Users: {:?}", users);
    
    // Update the user
    user.name = "Jane Doe".to_string();
    user = user.update(&client).await?;
    println!("Updated user: {:?}", user);
    
    // Delete the user
    user.delete(&client).await?;
    println!("User deleted");
    
    Ok(())
}
```

## Resource Definition

Define your resources using the `Resource` derive macro:

```rust
use serde::{Deserialize, Serialize};
use vamo::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/posts")]
struct Post {
    id: Option<u64>,
    title: String,
    content: String,
    user_id: u64,
    created_at: Option<String>,
}
```

## CRUD Operations

### Create

```rust
let mut post = Post {
    id: None,
    title: "Hello, Vamo!".to_string(),
    content: "This is a test post.".to_string(),
    user_id: 1,
    created_at: None,
};

// Create a new post
post = post.create(&client).await?;
```

### Read

```rust
// Get a post by ID
let post = Post::get(1, &client).await?;

// Get all posts
let posts = Post::list(&client).await?;

// With query parameters
use std::collections::HashMap;
let mut query = HashMap::new();
query.insert("user_id", "1");
let user_posts = Post::list_with_params(&client, &query).await?;
```

### Update

```rust
// Update a post
post.title = "Updated Title".to_string();
let updated_post = post.update(&client).await?;
```

### Delete

```rust
// Delete a post
post.delete(&client).await?;
```

## Relationships

```rust
#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/users")]
struct User {
    id: Option<u64>,
    name: String,
    email: String,
}

impl User {
    // Define a custom method to get user's posts
    pub async fn posts(&self, client: &vamo::Client) -> Result<Vec<Post>, vamo::Error> {
        Post::list_with_params(client, &[("user_id", &self.id.unwrap().to_string())]).await
    }
}

// Usage
let user = User::get(1, &client).await?;
let user_posts = user.posts(&client).await?;
```

## Middleware

```rust
use vamo::middleware::{Middleware, Next};
use deboa::Request;

struct AuthMiddleware {
    token: String,
}

#[async_trait::async_trait]
impl Middleware for AuthMiddleware {
    async fn handle(&self, req: Request, next: Next<'_>) -> Result<deboa::Response, deboa::Error> {
        next.run(req.header("Authorization", format!("Bearer {}", self.token))).await
    }
}

// Create a client with middleware
let client = vamo::Client::builder("https://api.example.com")
    .with(AuthMiddleware { token: "secret-token".to_string() })
    .build();
```

## Error Handling

```rust
use vamo::Error as VamoError;

match user.delete(&client).await {
    Ok(_) => println!("User deleted successfully"),
    Err(VamoError::NotFound) => eprintln!("User not found"),
    Err(VamoError::Unauthorized) => eprintln!("Authentication required"),
    Err(e) => eprintln!("An error occurred: {}", e),
}
```

## Features

- `json`: Enable JSON support (requires `serde_json`)
- `form`: Enable URL-encoded form support (requires `serde_urlencoded`)
- `multipart`: Enable multipart form data support
- `uuid`: Enable UUID support
- `chrono`: Enable date/time support with chrono

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vamo).
