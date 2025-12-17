# Deboa Macros

[![Crates.io downloads](https://img.shields.io/crates/d/deboa-macros)](https://crates.io/crates/deboa-macros) [![crates.io](https://img.shields.io/crates/v/deboa-macros?style=flat-square)](https://crates.io/crates/deboa-macros) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/deboa-macros) [![Documentation](https://docs.rs/deboa-macros/badge.svg)](https://docs.rs/deboa-macros/latest/deboa-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa-macros) 


**deboa-macros** is a collection of macros for deboa.
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
deboa-macros = "0.0.1"
```

## Usage

### other macros

```rust
use deboa::errors::DeboaError;
use deboa_macros::{fetch, get, post, delete};
use deboa_extras::http::serde::json::JsonBody;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

let mut client = Deboa::new();

// fetch macro
let response: Vec<Post> = fetch!("https://jsonplaceholder.typicode.com/posts", JsonBody, Vec<Post>, &mut client);

// get macro, returning posts serialized as json
let response: Vec<Post> = get!("https://jsonplaceholder.typicode.com/posts", JsonBody, Vec<Post>, &mut client);

//get macro, returning text
let response: String = get!("https://rust-lang.org", &mut client);

// post macro
let response = post!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts", &mut client);

// delete macro
let response = delete!("https://jsonplaceholder.typicode.com/posts/1", &mut client);

```

## Notes

It is not possible to use the same name for different operations.
Please keep struct names unique and in separate modules if possible.

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
