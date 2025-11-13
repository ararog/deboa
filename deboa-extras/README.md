# Deboa Extras

This crate provides additional features for Deboa like compression and serialization.

## Install

`cargo add deboa-extras`

## Features

- compression (gzip, deflate and brotli)
- serialization (json, xml, msgpack)
- sse
- websockets

## Usage

### Decompression

```rust
use deboa::{Deboa, errors::DeboaError, interceptor::DeboaCatcher, request::DeboaRequest};
use deboa_extras::{
    interceptor::encoding::EncodingCatcher,
    io::brotli::BrotliDecompressor,
    http::serde::json::JsonBody
};

let encoding_catcher = EncodingCatcher::register_decoders(vec![Box::new(BrotliDecompressor)]);

let mut client = Deboa::builder()
  .catch(encoding_catcher)
  .build()?

let posts = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts/1")?
  .send_with(&mut client)
  .await?
  .body_as(JsonBody)?;

println!("{:?}", posts.raw_body());
```

### Serialization

```rust
use deboa::{Deboa, errors::DeboaError, request::post};
use deboa_extras::http::serde::json::JsonBody;

let client = Deboa::new();

let data = Post {
    id: 1,
    title: "title".to_string(),
    body: "body".to_string(),
    user_id: 1,
};

let response = post("https://jsonplaceholder.typicode.com/posts/1")?
  .body_as(JsonBody, data)?
  .send_with(client)
  .await?;

println!("Response Status Code: {}", response.status());
```

### SSE

```rust
use deboa::{Deboa, Result};
use deboa_extras::http::sse::response::{IntoEventStream};

let mut client = Deboa::new();

let response = client.execute("https://sse.dev/test").await?.into_event_stream();

// Poll events, until the connection is closed
// please note that this is a blocking call
while let Some(event) = response.next().await {
    println!("event: {}", event);
}

println!("Connection closed");
```

### Websockets

```rust
use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::ws::{
    io::socket::DeboaWebSocket,
    protocol::{self},
    request::WebsocketRequestBuilder,
    response::IntoWebSocket,
};

let mut client = Deboa::new();

let websocket = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
    .send_with(&mut client)
    .await?
    .into_websocket()
    .await;

while let Ok(()) = websocket.read_message().await {
    // Just keep checking messages
}
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
