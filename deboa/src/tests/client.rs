use crate::{default_protocol, Client, Result};

use deboa_tests::utils::JSONPLACEHOLDER;

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

#[tokio::test]
async fn test_shl() -> Result<()> {
    let client = Client::default();
    let request = &client << JSONPLACEHOLDER;
    let response = client
        .execute(request)
        .await?;

    assert_eq!(response.status(), 200);

    Ok(())
}
