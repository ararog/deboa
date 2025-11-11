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
//!
//! ## Examples
//!
//! ### Using Server-Sent Events (SSE)
//!
//! ```compile_fail
//! use deboa_extras::http::sse::SseRequest;
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut client = deboa::Deboa::new();
//!   let sse = SseRequest::get("https://example.com/events")?;
//!   let mut stream = sse.go(&mut client).await?;
//!
//!   while let Some(event) = stream.next().await {
//!     match event {
//!       Ok(event) => println!("Event: {:?}", event),
//!       Err(e) => eprintln!("Error: {}", e),
//!     }
//!   }
//!   Ok(())
//! }
//! ```
//!
//! ### Using WebSockets
//!
//! ```compile_fail
//! use deboa_extras::ws::WebSocketRequest;
//! use futures_util::{SinkExt, StreamExt};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   let mut client = deboa::Deboa::new();
//!   let (mut ws, _) = WebSocketRequest::connect("wss://echo.websocket.org").go(&mut client).await?;
//!
//!   // Send a message
//!   ws.send("Hello, WebSocket!".into()).await?;
//!
//!   // Receive messages
//!   while let Some(msg) = ws.next().await {
//!     match msg {
//!       Ok(msg) => println!("Received: {:?}", msg),
//!       Err(e) => eprintln!("Error: {}", e),
//!     }
//!   }
//!   Ok(())
//! }
//! ```

pub mod catcher;
pub mod errors;
pub mod http;

#[cfg(feature = "compression")]
pub mod io;
#[cfg(test)]
mod tests;

#[cfg(feature = "websockets")]
pub mod ws;
