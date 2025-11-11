---
layout: default
title: Deboa Macros - Procedural Macros
nav_order: 5
---

# Deboa Macros

A collection of procedural macros to simplify working with the Deboa HTTP client.

## Features

- `#[derive(IntoRequest)]`: Automatically convert a struct into an HTTP request
- `#[derive(FromResponse)]`: Automatically parse an HTTP response into a struct
- `#[derive(QueryParams)]`: Convert a struct into URL query parameters
- `#[derive(FormData)]`: Convert a struct into form data
- `#[derive(JsonBody)]`: Convert a struct into a JSON request body

## Installation

```toml
[dependencies]
deboa-macros = "0.1.0"
```

## Usage

### IntoRequest and FromResponse

```rust
use deboa_macros::{IntoRequest, FromResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, IntoRequest, FromResponse)]
#[deboa(method = "POST", path = "/users")]
struct CreateUserRequest {
    name: String,
    email: String,
    #[deboa(header = "X-Request-ID")]
    request_id: String,
}

#[derive(Debug, Serialize, Deserialize, FromResponse)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = deboa::Deboa::new();
    
    let request = CreateUserRequest {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        request_id: "abc123".to_string(),
    };
    
    // Convert the request into an HTTP request
    let http_request = request.into_request("https://api.example.com")?;
    
    // Send the request
    let user: User = http_request
        .go(&client)
        .await?
        .body()?;
    
    println!("Created user: {:?}", user);
    
    Ok(())
}
```

### QueryParams

```rust
use deboa_macros::QueryParams;
use serde::Serialize;

#[derive(Debug, Serialize, QueryParams)]
struct UserQuery {
    name: Option<String>,
    email: Option<String>,
    #[query(rename = "sort_by")]
    sort_by: Option<String>,
    page: Option<u32>,
    per_page: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = UserQuery {
        name: Some("John".to_string()),
        email: None,
        sort_by: Some("name".to_string()),
        page: Some(1),
        per_page: Some(10),
    };
    
    let query_string = query.to_query_string()?;
    assert_eq!(query_string, "name=John&sort_by=name&page=1&per_page=10");
    
    let url = format!("https://api.example.com/users?{}", query_string);
    println!("URL: {}", url);
    
    Ok(())
}
```

### FormData

```rust
use deboa_macros::FormData;
use serde::Serialize;

#[derive(Debug, Serialize, FormData)]
struct LoginForm {
    username: String,
    password: String,
    #[form(rename = "remember_me")]
    remember_me: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let form = LoginForm {
        username: "johndoe".to_string(),
        password: "s3cr3t".to_string(),
        remember_me: true,
    };
    
    let form_data = form.to_form_data()?;
    
    let client = deboa::Deboa::new();
    let response = deboa::post("https://api.example.com/login")
        .body(form_data)
        .go(&client)
        .await?;
    
    println!("Login response: {}", response.status());
    
    Ok(())
}
```

### JsonBody

```rust
use deboa_macros::JsonBody;
use serde::Serialize;

#[derive(Debug, Serialize, JsonBody)]
struct CreatePostRequest {
    title: String,
    content: String,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let post = CreatePostRequest {
        title: "Hello, World!".to_string(),
        content: "This is a test post.".to_string(),
        tags: vec!["rust".to_string(), "deboa".to_string()],
    };
    
    let client = deboa::Deboa::new();
    let response = deboa::post("https://api.example.com/posts")
        .body(post.into_body()?)
        .go(&client)
        .await?;
    
    println!("Created post: {}", response.status());
    
    Ok(())
}
```

## Attribute Macros

### `#[deboa(method = "...", path = "...")]`

Specify the HTTP method and path for a request.

### `#[deboa(header = "...")]`

Specify a header for a request field.

### `#[query(rename = "...")]`

Rename a field in the query string.

### `#[form(rename = "...")]`

Rename a field in form data.

## Features

- `derive`: Enable derive macros (enabled by default)
- `serde`: Enable serde integration (enabled by default)
- `json`: Enable JSON support
- `form`: Enable form data support

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-macros).
