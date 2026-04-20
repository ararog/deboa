# Vamo

[![Crates.io downloads](https://img.shields.io/crates/d/vamo)](https://crates.io/crates/vamo) [![crates.io](https://img.shields.io/crates/v/vamo?style=flat-square)](https://crates.io/crates/vamo) [![Build Status](https://github.com/ararog/deboa/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/ararog/deboa/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vamo) [![Documentation](https://docs.rs/vamo/badge.svg)](https://docs.rs/vamo/latest/vamo) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ararog/deboa/blob/main/LICENSE.md) ![Codecov](https://img.shields.io/codecov/c/github/ararog/deboa) 

**vamo** ("Let's go" in portuguese) is a rest wrapper for deboa. Vamo is a key part of the deboa ecosystem, allowing bora macro to generate api clients.

## Features

- all deboa features
- set base url only once, change it when needed
- request data only by specifying path
- resource trait to make requests using any struct (experimental)

## Install

Either run from command line:

`cargo add vamo deboa-smol`

Or add to your `Cargo.toml`:

```toml
vamo = "0.0.1"
deboa-smol = "0.1.0"
```

## Usage

```rust, compile_fail
use vamo::Vamo;

let vamo = Vamo::<deboa_smol::Client>::new("https://api.example.com")?;
let response = vamo
    .get("/users")?
    .send()
    .await?;
```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

MIT

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
