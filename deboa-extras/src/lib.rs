//! # Deboa Extras
//!
//! `deboa-extras` is an extension crate for the `deboa` HTTP client that provides additional
//! functionality and utilities for working with HTTP requests and responses.
//!
//! ## Features
//!
//! - **HTTP Utilities**: Additional HTTP-related functionality including:
//!   - Server-Sent Events (SSE) support
//!   - WebSocket client (when `websockets` feature is enabled)
//!   - Enhanced serialization/deserialization
//! - **Compression**: Support for compressed request/response bodies (when `compression` feature is enabled)
//! - **Error Handling**: Extended error types and utilities
//! - **Catchers**: Pre-built error handlers for common scenarios
//!
//! ## Crate Features
//!
//! - `compression`: Enables compression support (gzip, deflate, brotli)
//! - `websockets`: Enables WebSocket client functionality
//! - `json`: Enables JSON serialization/deserialization
//! - `msgpack`: Enables MessagePack serialization/deserialization
//! - `xml`: Enables XML serialization/deserialization
//! - `yaml`: Enables YAML serialization/deserialization
//! - `flex`: Enables flexbuffers serialization/deserialization
//!
//! ## Examples
//!
//! ### Using Server-Sent Events (SSE)
//!
//! ```compile_fail
//! use deboa::{Client, Result};
//! use deboa_extras::http::sse::SseRequest;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   let mut client = Client::default();
//!   let response = client
//!     .execute("https://sse.dev/test")
//!     .await?
//!     .into_event_stream();
//!
//!   // Poll events, until the connection is closed
//!   // please note that this is a blocking call
//!   while let Some(event) = response.next().await {
//!     println!("event: {}", event);
//!   }
//!
//!   println!("Connection closed");
//!   Ok(())
//! }
//! ```
//!
//! ### Using WebSockets
//!
//! ```compile_fail
//! use deboa::{Client, Result};
//! use deboa_extras::ws::{
//!     io::socket::DeboaWebSocket,
//!     protocol::{self},
//!     request::WebsocketRequestBuilder,
//!     response::IntoWebSocket,
//! };
//!
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!   let mut client = Client::default();
//!   let websocket = WebsocketRequestBuilder::connect("wss://echo.websocket.org")
//!     .send_with(&mut client)
//!     .await?;
//!
//!   // Send a message
//!   websocket.send_text("Hello, WebSocket!".into()).await?;
//!
//!   // Receive messages
//!   while let Ok(Some(msg)) = websocket.read_message().await {
//!       match msg {
//!           protocol::Message::Text(text) => println!("Received text: {}", text),
//!           protocol::Message::Binary(data) => println!("Received binary data: {} bytes", data.len()),
//!       }
//!   }
//!   Ok(())
//! }
//! ```

//pub mod catcher;
pub mod errors;
pub mod http;

#[cfg(any(feature = "deflate", feature = "gzip", feature = "brotli"))]
//pub mod io;
#[cfg(test)]
mod tests;

#[cfg(feature = "websockets")]
pub mod ws;
