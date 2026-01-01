---
layout: default
title: Deboa Macros - Procedural Macros
nav_order: 5
---

# Deboa Macros

Collection of procedural macros for Deboa HTTP client. It is close equivalent to
apisauce for axios, where one macro does it all, from request to response.
It used to be the home of bora macro, which has been moved to vamo-macros crate.

Available macros includes:

- `fetch!`
- `get!`
- `post!`
- `delete!`
- `put!`
- `patch!`

## Installation

```toml
[dependencies]
deboa-macros = "0.1.0"
```

## Usage

### get!

```rust
let mut client = Client::new();

let response: Vec<Post> = get!(
  "https://jsonplaceholder.typicode.com/posts", 
  JsonBody, 
  Vec<Post>, 
  &mut client
);
```

### post!

```rust
let data = serde_json::json!({"title": "foo", "body": "bar", "userId": 1});
let response = post!(
    data, 
    JsonBody, 
    "https://jsonplaceholder.typicode.com/posts", 
    &mut client
);
```

### fetch!

```rust
let response: Vec<Post> = fetch!(
    "https://jsonplaceholder.typicode.com/posts", 
    JsonBody, 
    Vec<Post>, 
    &mut client
);
```

### delete!

```rust
let response = delete!("https://jsonplaceholder.typicode.com/posts/1", &mut client);
```

### put!

```rust
let data = serde_json::json!({"id": 1, "title": "foo", "body": "bar", "userId": 1});
let response = put!(
    data, 
    JsonBody, 
    "https://jsonplaceholder.typicode.com/posts/1", 
    &mut client
);
```

### patch!

```rust
let data = serde_json::json!({"title": "foo"});
let response = patch!(
    data, 
    JsonBody, 
    "https://jsonplaceholder.typicode.com/posts/1", 
    &mut client
);
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-macros).
