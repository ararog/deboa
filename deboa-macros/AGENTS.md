## Project Overview

This crate provides macros for the deboa HTTP client. Easily make HTTP requests with minimal boilerplate.
Please note this also provides bora macro for generating API clients.

## Installing

Add this to your `Cargo.toml`:

```toml
[dependencies]
deboa-macros = "0.0.9"
```

## Usage

### bora macro

The `bora` macro generates type-safe API client struct with methods for defined endpoints.

It allows you to define API endpoints declaratively and automatically generates corresponding methods.

The macro supports various HTTP methods (GET, POST, PUT, PATCH, DELETE) and can handle path parameters, query parameters, and different response formats like JSON, XML, and others.


```rust
use deboa::errors::DeboaError;
use deboa_macros::bora;
use vamo::Vamo;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
    api(
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json")
    )
)]
pub struct PostService;

let client = Vamo::new("https://jsonplaceholder.typicode.com");

let mut post_service = PostService::new(client);

let post = post_service.get_by_id(1).await?;

println!("id...: {}", post.id);
println!("title: {}", post.title);

Ok(())
```

### ṕrocedural macros (get, fetch, post, put, patch and delete)


The procedural macros provide convenient shortcuts for common HTTP operations without needing to define a full API client structure.

They are useful for quick HTTP requests where you don't need the full type-safe client generation.

They support various HTTP methods and can handle different response types.


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

## Available Features

The following features are available in `deboa-macros`:

- `json`: Enable JSON serialization support
- `xml`: Enable XML serialization support
- `msgpack`: Enable MessagePack serialization support

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
