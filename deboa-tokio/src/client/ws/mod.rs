//!
//! WebSocket support for deboa-extras.
//! Provides WebSocket client functionality with message encoding/decoding.
//! Supports text and binary message types with automatic serialization.
//! Includes support for various serialization formats through the serialization feature.
//! Requires the `websockets` feature to be enabled.
//!
//!
//! ## Example
//! ```rust, compile_fail
//! use deboa::{Client, Result, request::DeboaRequestBuilder};
//! use deboa_extras::ws::{
//!     io::socket::DeboaWebSocket,
//!     protocol::{self},
//!     request::WebsocketRequestBuilder,
//!     response::IntoWebSocket,
//! };
//!
//! let mut client = Client::new();
//!
//! let websocket = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
//!     .send_with(&mut client)
//!     .await?
//!     .into_websocket()
//!     .await;
//!
//! loop {
//!     select! {
//!         outgoing_message = websocket.read_message() => {
//!             if let Err(message) = outgoing_message {
//!                 println!("Failed to read message from echo server: {}", message);
//!
//!                 output.send(Event::Disconnected).await;
//!                 break;
//!             }
//!
//!             match outgoing_message.unwrap() {
//!                 Some(message) => {
//!                     if let protocol::Message::Text(message) = message {
//!                         output
//!                             .send(Event::MessageReceived(Message::User(
//!                                 format!("Server: {}", message).to_string(),
//!                             )))
//!                                        .await;
//!                      }
//!                 }
//!                 None => {
//!                     output.send(Event::Disconnected).await;
//!                     break;
//!                 }
//!             }
//!         }
//!
//!         incoming_message = input.next() => {
//!             if let Some(message) = incoming_message {
//!                 let result = websocket
//!                   .write_message(protocol::Message::Text(message.to_string()))
//!                   .await;
//!                 if result.is_err() {
//!                     output.send(Event::Disconnected).await;
//!                     break;
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Modules
//!
//! * `io` - WebSocket I/O operations
//! * `protocol` - WebSocket protocol handling
//! * `request` - WebSocket request building
//! * `response` - WebSocket response parsing
//!
pub mod io;
pub mod protocol;
pub mod request;
pub mod response;
