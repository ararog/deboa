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

## Installation

```toml
[dependencies]
vamo = { version = "0.0.5" }
```

## Quick Start

```rust
use serde::{Deserialize, Serialize};
use vamo::Vamo;

// Define your resource
#[derive(Debug, Serialize, Deserialize, Resource)]
struct User {
    id: Option<u64>,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client
    let client = Vamo::new("https://api.example.com");
    
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

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vamo).
