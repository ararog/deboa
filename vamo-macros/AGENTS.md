# Project Overview

This crate provides procedural macros to automate resource trait generation for vamo.

## Installing

Add this to your `Cargo.toml`:

```toml
[dependencies]
vamo-macros = "0.0.4"
```

## Usage

The macros automatically generate resource traits based on your struct definitions.

## Example

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

## Example Output

The above macro usage will generate the necessary trait implementations for the `User` struct, including methods for serialization, deserialization, and HTTP handling.

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
