---
layout: default
title: Deboa Macros - Procedural Macros
nav_order: 5
---

## Deboa Macros

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

### get

```rust
let client = Client::new();

let response: Vec<Post> = get!(
  url => "https://jsonplaceholder.typicode.com/posts",
  res_body_ty => JsonBody,
  res_ty => Vec<Post>,
  client => &client
);
```

### post

```rust
let data = serde_json::json!({"title": "foo", "body": "bar", "userId": 1});
let response = post!(
    data => data,
    res_body_ty => JsonBody,
    url => "https://jsonplaceholder.typicode.com/posts",
    client => &client
);
```

### fetch

```rust
let response: Vec<Post> = fetch!(
    url => "https://jsonplaceholder.typicode.com/posts",
    res_body_ty => JsonBody,
    res_ty => Vec<Post>,
    client => &client
);
```

### delete

```rust
let response = delete!(
    url => "https://jsonplaceholder.typicode.com/posts/1",
    client => &client
);
```

### put

```rust
let data = serde_json::json!({"id": 1, "title": "foo", "body": "bar", "userId": 1});
let response = put!(
    data => data,
    res_body_ty => JsonBody,
    url => "https://jsonplaceholder.typicode.com/posts/1",
    client => &client
);
```

### patch

```rust
let data = serde_json::json!({"title": "foo"});
let response = patch!(
    data => data,
    res_body_ty => JsonBody,
    url => "https://jsonplaceholder.typicode.com/posts/1",
    client => &client
);
```

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/deboa-macros).
