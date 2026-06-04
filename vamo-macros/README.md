# Vamo Macros

[![Crates.io downloads](https://img.shields.io/crates/d/vamo-macros)](https://crates.io/crates/vamo-macros) [![crates.io](https://img.shields.io/crates/v/vamo-macros?style=flat-square)](https://crates.io/crates/vamo-macros) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vamo-macros) [![Documentation](https://docs.rs/vamo-macros/badge.svg)](https://docs.rs/vamo-macros/latest/vamo-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa)

Vamo macros is a collection of macros to make possible
use structs as resources to be sent over vamo as client.

## Features

- derive macro for resource trait implementation
- bora attribute macro for quick client creation

## Install

Either run from command line:

`cargo add vamo-macros vamo deboa-tokio`

Or add to your `Cargo.toml`:

```toml
vamo = "0.0.1"
vamo-macros = "0.0.1"
deboa-tokio = "0.1.0"
```

## Usage

### Resource macro

```rust
use deboa_extras::http::serde::json::JsonBody;
use deboa_tokio::Client;
use vamo_macros::Resource;
use vamo::{Vamo, ResourceMethod};

#[derive(Resource)]
#[name("posts")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

let mut vamo = Vamo::<Client>::new("https://api.example.com")?;

// post
let response = vamo
    .create(user)
    .await?
    .send()
    .await?;

// put
vamo.update(user)?
    .send()
    .await?;

// patch
vamo.edit(user)?
    .send()
    .await?;

// delete
vamo.remove(user)?
    .send()
    .await?;

```

### bora macro

```rust
use deboa::Result;
use deboa_tokio::Client;
use vamo::Vamo;
use vamo_macros::bora;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
    api(
        get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
        get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
    )
)]
pub struct PostService;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Vamo::<Client>::new("https://jsonplaceholder.typicode.com")?;
    let mut post_service = PostService::new(client);
    let post = post_service.get_by_id(1).await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);
    assert_eq!(post.id, 1);
    
    Ok(())
}
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  (LICENSE-MIT or https://opensource.org/licenses/MIT)

at your option.

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
