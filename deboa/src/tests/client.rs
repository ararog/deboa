use crate::Deboa;
use crate::errors::DeboaError;

use deboa_tests::utils::JSONPLACEHOLDER;

#[test]
fn test_set_connection_timeout() -> Result<(), DeboaError> {
    let api = Deboa::builder().connection_timeout(5).build();

    assert_eq!(api.connection_timeout, 5);

    Ok(())
}

#[test]
fn test_set_request_timeout() -> Result<(), DeboaError> {
    let api = Deboa::builder().request_timeout(5).build();

    assert_eq!(api.request_timeout, 5);

    Ok(())
}

#[test]
fn test_set_protocol() -> Result<(), DeboaError> {
    let api = Deboa::builder().protocol(crate::HttpVersion::Http1).build();

    assert_eq!(api.protocol, crate::HttpVersion::Http1);

    Ok(())
}

#[tokio::test]
async fn test_shl() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    let request = &client << JSONPLACEHOLDER;
    let response = client.execute(request).await?;

    assert_eq!(response.status(), 200);

    Ok(())
}
