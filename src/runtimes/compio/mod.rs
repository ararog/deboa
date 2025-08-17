#![deny(warnings)]
#![warn(rust_2018_idioms)]

use std::str::FromStr;

use anyhow::Result;
use cyper_core::{HttpStream, TlsBackend};
use hyper::client::conn::http1::{Connection, SendRequest};
use hyper::Uri;
use url::Url;

pub async fn get_connection(
    url: &Url,
) -> Result<(SendRequest<String>, Connection<HttpStream, String>)> {
    let uri = Uri::from_str(url.as_str()).unwrap();

    let stream = HttpStream::connect(uri, TlsBackend::default()).await?;

    Ok(hyper::client::conn::http1::handshake(stream).await?)
}
