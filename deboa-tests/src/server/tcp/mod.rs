#[cfg(all(
    feature = "smol-rt",
    feature = "smol-rust-tls",
    any(feature = "http1", feature = "http2")
))]
pub mod smol;
#[cfg(all(
    feature = "tokio-rt",
    feature = "tokio-rust-tls",
    any(feature = "http1", feature = "http2")
))]
pub mod tokio;

use crate::server::Server;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;

pub trait TcpServer: Server<Incoming, Full<Bytes>> {}
