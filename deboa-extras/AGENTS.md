## Project Overview

This crate provides extra functionality for the deboa HTTP client, including serialization, compression, websockets, streams, and middleware support.

## Installing

Add this to your `Cargo.toml`:

```toml
[dependencies]
deboa-extras = "0.0.4"
```

## Available Features

The following features are available in `deboa-extras`:

- `serialization`: Enable serialization support (JSON, XML, MessagePack, YAML and FlexBuffers)
- `compression`: Enable compression support (GZIP, Brotli, Deflate)
- `websockets`: Enable WebSocket support
- `sse`: Enable Server-Sent Events support
- `utils`: Enable utility functions and helpers
- `tokio`: Enable Tokio runtime support
- `smol`: Enable Smol runtime support

## Coding guidelines

- Follow Rust best practices and idioms
- Use descriptive variable and function names
- Write comprehensive documentation for public APIs
- Include examples in documentation where appropriate
- Maintain backward compatibility when possible

## Documentation instructions

- Keep documentation up to date with code changes
- Use clear and concise language
- Include code examples where appropriate

## Testing instructions

- Find the CI plan in the `.github/workflows` folder.
- From the crate root you can just call `cargo test`. The commit should pass all tests before you merge.

## PR instructions

- Title format: [<crate_name>] <Title>
- Always run `cargo fmt` and `cargo test` before committing.
- Keep changes focused and small.
- Include a brief description of the changes in the PR.
- Reference any related issues or discussions.
- Ensure all tests pass and code is properly formatted.
- Follow the existing code style and conventions.
- Add tests for new functionality when appropriate.
- Update documentation if needed.
- Keep commit messages clear and descriptive.
- Squash commits when appropriate for cleaner history.
- Request review from a maintainer before merging.
- Ensure CI checks pass before merging.
- Follow semantic versioning for releases.
