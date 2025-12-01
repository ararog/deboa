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
//! use deboa::{Deboa, Result, request::DeboaRequestBuilder};
//! use deboa_extras::ws::{
//!     io::socket::DeboaWebSocket,
//!     protocol::{self},
//!     request::WebsocketRequestBuilder,
//!     response::IntoWebSocket,
//! };
//!
//! let mut client = Deboa::new();
//!
//! let websocket = DeboaRequestBuilder::websocket("wss://echo.websocket.org")?
//!     .send_with(&mut client)
//!     .await?
//!     .into_websocket()
//!     .await;
//!
//! while let Ok(()) = websocket.read_message().await {
//!     // Just keep checking messages
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
