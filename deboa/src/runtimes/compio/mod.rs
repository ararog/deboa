#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::str::FromStr;

use anyhow::Result;
use bytes::Bytes;
use cyper_core::{HttpStream, TlsBackend};
#[cfg(feature = "http1")]
use hyper::client::conn::http1::{Connection, SendRequest};
#[cfg(feature = "http2")]
use hyper::client::conn::http2::{Connection, SendRequest};
use hyper::Uri;
use url::Url;

pub async fn get_connection(
    url: &Url,
) -> Result<(
    SendRequest<http_body_util::Full<Bytes>>,
    Connection<HttpStream, http_body_util::Full<Bytes>>,
)> {
    let uri = Uri::from_str(url.as_str()).unwrap();

    let stream = HttpStream::connect(uri, TlsBackend::default()).await?;

    #[cfg(feature = "http1")]
    return Ok(hyper::client::conn::http1::handshake(stream).await?);

    #[cfg(feature = "http2")]
    return Ok(hyper::client::conn::http2::handshake(stream).await?);
}
