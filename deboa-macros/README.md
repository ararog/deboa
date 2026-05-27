# Deboa Macros

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-macros)](https://crates.io/crates/deboa-macros) [![crates.io](https://img.shields.io/crates/v/deboa-macros?style=flat-square)](https://crates.io/crates/deboa-macros) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-macros) [![Documentation](https://docs.rs/deboa-macros/badge.svg)](https://docs.rs/deboa-macros/latest/deboa-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa-macros) 

**deboa-macros** is a collection of macros for deboa. It is close equivalent to
apisauce for axios, where one macro does it all, from request to response.
It used to be the home of bora macro, which has been moved to vamo-macros crate.

## Features

- json
- xml
- msgpack

## Install

Either run from command line:

`cargo add deboa-macros`

Or add to your `Cargo.toml`:

```toml
deboa-macros = "0.0.8"
```

## Usage

### other macros

```rust,no_run
use deboa::errors::DeboaError;
use deboa_macros::{fetch, get, post, delete};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tokio::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let client = Client::default();

    // fetch macro
    let response: Vec<Post> = fetch!(
        url => "https://jsonplaceholder.typicode.com/posts", 
        client => &client, 
        res_body_ty => JsonBody, 
        res_ty => Vec<Post>
    );

    // get macro, returning posts serialized as json
    // let response: Vec<Post> = get!(
    //     url => "https://jsonplaceholder.typicode.com/posts", 
    //     client => &client, 
    //     res_body_ty => JsonBody, 
    //     res_ty => Vec<Post>
    // );

    // get macro, returning text
    // let response: String = get!(
    //     url => "https://rust-lang.org", 
    //     client => &client
    // );

    // get macro with headers
    // let response: String = get!(
    //     url => "https://rust-lang.org", 
    //     headers => vec![("User-Agent", "deboa")], 
    //     client => &client
    // );

    // post macro
    // let response = post!(
    //     data => data, 
    //     res_body_ty => JsonBody, 
    //     url => "https://jsonplaceholder.typicode.com/posts", 
    //     client => &client
    // );

    // delete macro
    // let response = delete!(
    //     url => "https://jsonplaceholder.typicode.com/posts/1", 
    //     client => &client
    // );
    
    Ok(())
}
```

## License

MIT and Apache-2.0

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
