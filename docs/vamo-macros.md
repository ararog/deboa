---
layout: default
title: Vamo Macros - bora amd resource macros
nav_order: 7
---

# Vamo Macros

Procedural macros for the Vamo crate, enabling seamless integration with the Deboa HTTP client.

## Features

- `#[derive(Resource)]`: Automatically implement the `Resource` trait for structs
- `#[bora]`: Automatically implement a Vamo client in empty structs

## Installation

```toml
[dependencies]
vamo-macros = "0.0.5"
```

## Usage

### Resource derive macro

```rust
use serde::{Deserialize, Serialize};
use vamo_macros::Resource;

#[derive(Debug, Serialize, Deserialize, Resource)]
struct User {
    id: Option<u64>,
    name: String,
    email: String,
}
```

### Bora macro

Please refer to this page for more information: [Vamo Bora](./vamo-bora.md)

## API Reference

For detailed API documentation, see the [docs.rs page](https://docs.rs/vamo-macros).
