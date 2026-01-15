#[cfg(all(feature = "smol-rt", feature = "smol-rust-tls"))]
pub mod smol;
#[cfg(all(feature = "tokio-rt", feature = "tokio-rust-tls"))]
pub mod tokio;

use crate::server::Server;
use bytes::Bytes;
use http_body_util::{BodyExt, Full};

pub trait UdpServer: Server<Full<Bytes>, Full<Bytes>> {}
