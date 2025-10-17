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

let client = Deboa::builder()
  .catch(encoding_catcher)
  .build()?

let posts = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts/1")?
  .go(client)
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
  .go(client)
  .await?;

println!("Response Status Code: {}", response.status());
```

### SSE

```rust
use deboa::{Deboa, Result};
use deboa_extras::http::sse::response::{EventHandler, IntoStream};

let mut client = Deboa::new();

let response = client.execute("https://sse.dev/test").await?.into_stream();

let handler = SSEHandler;

response.poll_event(handler).await?;

println!("Connection closed");
```

Implement the event handler:

```rust
pub struct SSEHandler;

#[deboa::async_trait]
impl EventHandler for SSEHandler {
    async fn on_event(&mut self, event: &str) -> Result<()> {
        println!("event: {}", event);
        Ok(())
    }
}
```

### Websockets

```rust
use deboa::{Deboa, Result, request::DeboaRequestBuilder};
use deboa_extras::http::ws::{
    protocol::{Message, MessageHandler, WebSocketRead, WebSocketWrite},
    request::WebsocketRequestBuilder,
    response::IntoStream,
};

let mut client = Deboa::new();

let (tx, mut rx) = channel::<Message>(100);

let handler = ChatHandler { tx: tx.clone() };

let response = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
    .go(&mut client)
    .await?
    .into_stream(handler)
    .await;

let (mut reader, mut writer) = response.split();

while let Ok(()) = reader.read_message().await {
    // Just keep checking messages
}
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
