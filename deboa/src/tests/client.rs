use std::sync::Arc;

use url::Url;

use crate::{default_protocol, Client, Result};

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

#[test]
fn test_set_connection_timeout() -> Result<()> {
    let api = Client::builder()
        .connection_timeout(5)
        .build();

    assert_eq!(api.connection_timeout, 5);

    Ok(())
}

#[test]
fn test_set_request_timeout() -> Result<()> {
    let api = Client::builder()
        .request_timeout(5)
        .build();

    assert_eq!(api.request_timeout, 5);

    Ok(())
}

#[test]
fn test_set_protocol() -> Result<()> {
    let api = Client::builder()
        .protocol(default_protocol())
        .build();

    assert_eq!(api.protocol, default_protocol());

    Ok(())
}

#[test]
fn test_set_skip_cert_verification() -> Result<()> {
    let api = Client::builder()
        .skip_cert_verification(true)
        .build();

    assert!(api.skip_cert_verification);

    Ok(())
}

async fn shl() -> Result<()> {
    let client = Client::default();
    let request = &client << "https://httpbin.org/get";

    assert_eq!(request.url(), Arc::new(Url::parse("https://httpbin.org/get").unwrap()));

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_shl() -> Result<()> {
    shl().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_shl() -> Result<()> {
    shl().await
}
