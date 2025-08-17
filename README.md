# deboa

A very simple and straightforward HTTP client.

The goal is to provide a simple and easy to use HTTP, very
similar to apisauce for nodejs/javascript.

## Install

deboa = { version = "0.0.4" }

## Features

- tokio-rt (default)
- smol-rt

## Usage

```
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com");

let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```