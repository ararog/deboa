# Vamo Macros

[![Crates.io downloads](https://img.shields.io/crates/d/vamo-macros)](https://crates.io/crates/vamo-macros) [![crates.io](https://img.shields.io/crates/v/vamo-macros?style=flat-square)](https://crates.io/crates/vamo-macros) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vamo-macros) [![Documentation](https://docs.rs/vamo-macros/badge.svg)](https://docs.rs/vamo-macros/latest/vamo-macros) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md)  ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa) 


Vamo macros is a collection of macros to make possible
use structs as resources to be sent over vamo as client.

## Usage

```rust
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

let mut vamo = Vamo::new("https://api.example.com")?;
let response = vamo.create(user).await?;
```

## Features

- derive macro for resource trait implementation

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

MIT

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
