# deboa

[![crates.io](https://img.shields.io/crates/v/deboa?style=flat-square)](https://crates.io/crates/deboa) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) [![Documentation](https://docs.rs/deboa/badge.svg)](https://docs.rs/deboa/latest/deboa)

## Description

**deboa** is a straightforward, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

## Features

- built-in json, xml and msgpack support
- easily add, remove and update headers
- helpers to add basic and bearer auth
- set base url only once, change it when needed
- request data only by specifying path
- set retries and timeout
- middleware support
- compression support (gzip, deflate, br)
- bora macro to easily create api clients
- cookies support
- comprehensive error handling
- runtime compatibility (tokio, smol, compio)
- http1 support (http2 coming soon) 

## Install

```rust
deboa = { version = "0.0.5", features = ["http1", "middlewares",  "json", "tokio-rt"] }
```

## Crate features

- tokio-rt (default)
- smol-rt
- compio-rt
- json (default)
- xml
- msgpack
- http1 (default)
- http2 (coming soon)
- middlewares (default)

## Usage

### Serialize request and deserialize response using json

```rust
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com");

let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Serialize request and deserialize response using xml

```rust
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com");

let posts: Vec<Post> = api.get("/posts").await?.xml::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Adding headers

```rust
use deboa::Deboa;
use http::header;

let api = Deboa::new("https://jsonplaceholder.typicode.com");
api.add_header(header::CONTENT_TYPE, "application/json");
let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Adding bearer auth

```rust
use deboa::Deboa;
use http::header;

let api = Deboa::new("https://jsonplaceholder.typicode.com");
api.add_bearer_auth("token");
let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Adding basic auth

```rust
use deboa::Deboa;
use http::header;

let api = Deboa::new("https://jsonplaceholder.typicode.com");
api.add_basic_auth("username", "password");
let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Change request base url

```rust
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com");
api.set_base_url("https://jsonplaceholder.typicode.com");
let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

### Adding middleware

```rust
use deboa::{Deboa, DeboaMiddleware};

struct MyMiddleware;

impl DeboaMiddleware for MyMiddleware {
    fn on_request(&self, request: &mut Request) {
        // Do something with the request
    }

    fn on_response(&self, response: &mut Response) {
        // Do something with the response
    }
}

let api = Deboa::new("https://jsonplaceholder.typicode.com");
api.add_middleware(MyMiddleware);
let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
