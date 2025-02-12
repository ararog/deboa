# deboa

A very simple and straightforward HTTP client.

The goal is to provide a simple and easy to use HTTP, very
similar to apisauce for nodejs/javascript.

## Install

deboa = { version = "0.0.1" }

## Usage

```
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

let res = api.get("/posts").await;

let posts: std::result::Result<Post, serde_json::Error> =
    serde_json::from_reader(res.unwrap().reader());

println!("posts: {:#?}", posts);
```