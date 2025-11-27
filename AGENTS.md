## Project Overview

This project is a Rust library named `deboa`, a straightforward, non opinionated, developer-centric HTTP client library for Rust. It offers a rich array of modern features—from flexible authentication and serialization formats to runtime compatibility and middleware support—while maintaining simplicity and ease of use. It’s especially well-suited for Rust projects that require a lightweight, efficient HTTP client without sacrificing control or extensibility.

The library is highly configurable and supports:

- easily add, remove and update headers
- helpers to add basic and bearer auth
- set retries and timeout
- pluggable catchers (interceptors)
- pluggable compression (gzip, deflate, br)
- pluggable serialization (json, xml, msgpack)
- cookies support
- urlencoded and multipart forms
- comprehensive error handling
- response streaming
- upgrade support (websocket, etc.)
- runtime compatibility (tokio and smol)
- http1/2 support 

The library is modular, and features can be enabled or disabled via Cargo
features to keep the binary size small.

## Related Projects

### deboa-bora

A crate with bora macro, for easy rest client generation.

### deboa-extras

Pluggable compression/decompression, serializers and catchers.

### deboa-macros

A crate with set of convenience macros.

### deboa-tests

A crate with testing utilities.

### examples

Examples of how to use deboa.

### vamo

Nice wrapper on top of deboa for dry rest client.

### vamo-macros

Vamo macros is a collection of macros to make possible
use structs as resources to be sent over vamo as client.

### uget

A simple command line tool to download files from the web.
Available at https://github.com/ararog/uget

## Building and Running

The project is a standard Rust library. It can be built and tested using
`cargo`.

### Building

To build the library, run:

```sh
cargo build
```

To build the library with all features, run:

```sh
cargo build --all-features
```

### Running Examples

The project includes a number of examples in the `examples` directory. To run an
example, use the `cargo run --example` command. For example, to run the
`interactive` example, which demonstrates most of the available features, run:

```sh
cargo run --example interactive
```

### Running Tests

To run the tests, run:

```sh
cargo test
```

To run the tests with all features, run:

```sh
cargo test --all-features
```

## Development Conventions

The project follows standard Rust conventions.

- **Formatting:** The code is formatted with `dprint fmt`. This will format all
  project files.
- **Linting:** The project uses `clippy` for linting.
- **Continuous Integration:** The project uses GitHub Actions for continuous
  integration. The configuration is in the `.github/workflows` directory.
- **Dependencies:** The project uses `dependabot` to keep dependencies up to
  date. The configuration is in the `.github/dependabot.yml` file.
- **Documentation:** The project has extensive documentation, which can be
  generated with `cargo doc`. The documentation is also available on
  [docs.rs](https://docs.rs/textwrap/).