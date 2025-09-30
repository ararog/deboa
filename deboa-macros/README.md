# Deboa Macros

**deboa-macros** is a collection of macros for deboa.
It used to be the home of bora macro, which has been moved to its own crate
but it will continue to exist in this crate for backwards compatibility.

## Install

`cargo add deboa-macros`

## Features

- json
- xml
- msgpack

## Usage

### bora

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
let response: Vec<Post> = fetch!("https://jsonplaceholder.typicode.com/posts", JsonBody, Vec<Post>, using &mut client);

// get macro
let response: Vec<Post> = get!("https://jsonplaceholder.typicode.com/posts", JsonBody, Vec<Post>, using &mut client);

// post macro
let response = post!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts", using &mut client);

// delete macro
let response = delete!("https://jsonplaceholder.typicode.com/posts", using &mut client);

```

## Notes

It is not possible to use the same name for different operations.
Please keep struct names unique and in separate modules if possible.

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
