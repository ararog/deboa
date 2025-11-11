---
layout: default
title: Vamo Macros - Resource Macros
nav_order: 7
---

# Vamo Macros

Procedural macros for the Vamo crate, enabling seamless integration with the Deboa HTTP client.

## Features

- `#[derive(Resource)]`: Automatically implement the `Resource` trait for structs
- `#[resource]`: Configure resource behavior and endpoints
- Custom field attributes for fine-grained control

## Installation

```toml
[dependencies]
vamo-macros = "0.1.0"
```

## Usage

### Basic Resource

```rust
use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/users")]
struct User {
    id: Option<u64>,
    name: String,
    email: String,
}
```

### Custom Endpoints

```rust
#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/articles")]
struct Article {
    id: Option<u64>,
    title: String,
    content: String,
    published: bool,
    
    #[resource(skip)]
    metadata: Option<serde_json::Value>,
}

// Custom implementation for the Article resource
impl Article {
    // Custom method to publish an article
    pub async fn publish(&mut self, client: &vamo::Client) -> Result<Self, vamo::Error> {
        self.published = true;
        self.update(client).await
    }
    
    // Custom endpoint to get published articles
    pub async fn published(client: &vamo::Client) -> Result<Vec<Self>, vamo::Error> {
        Self::list_with_params(client, &[("published", "true")]).await
    }
}
```

### Field Attributes

```rust
#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/products")]
struct Product {
    id: Option<u64>,
    name: String,
    
    #[resource(readonly)]
    created_at: Option<String>,
    
    #[resource(skip_deserialize)]
    temporary_data: Option<String>,
    
    #[resource(skip_serialize)]
    secret_token: Option<String>,
}
```

### Nested Resources

```rust
#[derive(Debug, Serialize, Deserialize, Resource)]
#[resource(path = "/users/{user_id}/posts")]
struct UserPost {
    id: Option<u64>,
    user_id: u64,
    title: String,
    content: String,
}

// Usage
let posts = UserPost::list_with_params(
    &client, 
    &[("user_id", "1")]
).await?;
```

## Attribute Reference

### Struct Attributes

- `#[resource(path = "/endpoint")]`: Set the base API endpoint for the resource
- `#[resource(primary_key = "field_name")]`: Specify a custom primary key field (default is `id`)
- `#[resource(version = "v1")]`: Set the API version prefix

### Field Attributes

- `#[resource(readonly)]`: Field is read-only and won't be included in create/update requests
- `#[resource(skip)]`: Field is completely ignored by the resource system
- `#[resource(skip_serialize)]`: Field is not included when serializing to JSON
- `#[resource(skip_deserialize)]`: Field is not set when deserializing from JSON
- `#[resource(default)]`: Use the default value when deserializing if the field is missing
- `#[resource(rename = "new_name")]`: Use a different name in the JSON representation

## Custom Serialization

```rust
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Resource)]
#[resource(path = "/custom")]
struct CustomResource {
    id: Option<u64>,
    
    #[serde(serialize_with = "serialize_custom", deserialize_with = "deserialize_custom")]
    data: CustomType,
}

#[derive(Debug)]
struct CustomType {
    value: String,
}

fn serialize_custom<S>(value: &CustomType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.value)
}

fn deserialize_custom<'de, D>(deserializer: D) -> Result<CustomType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(CustomType { value: s })
}
```

## Error Handling

```rust
use vamo::Error as VamoError;

match article.update(&client).await {
    Ok(updated) => println!("Updated: {:?}", updated),
    Err(VamoError::Validation(errors)) => {
        eprintln!("Validation errors: {:?}", errors);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Features

- `serde`: Enable serde integration (enabled by default)
- `json`: Enable JSON support (requires `serde_json`)
- `uuid`: Enable UUID support
- `chrono`: Enable date/time support with chrono

## Examples

See the [examples](../examples) directory for more usage examples.

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vamo-macros).
