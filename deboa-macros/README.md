# Deboa Macros

## Install

`cargo add deboa-macros`

## Features

- json
- xml
- msgpack

## Usage

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
