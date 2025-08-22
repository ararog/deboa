# deboa

A very simple and straightforward HTTP client.

The goal is to provide a simple and easy to use HTTP, very
similar to apisauce for nodejs/javascript.

## Install

deboa = { version = "0.0.5" }

## Features

- tokio-rt (default)
- smol-rt
- compio-rt
- json
- xml
- msgpack
- http1
- http2
- middlewares

## Usage

### Serialize request amd deserialize response using json

```
use deboa::Deboa;

let api = Deboa::new("https://jsonplaceholder.typicode.com");

let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

println!("posts: {:#?}", posts);
```