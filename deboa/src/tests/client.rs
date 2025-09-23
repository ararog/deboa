use crate::Deboa;
use crate::errors::DeboaError;

#[test]
fn test_set_retries() -> Result<(), DeboaError> {
    let api = Deboa::builder().retries(5).build();

    assert_eq!(api.retries, 5);

    Ok(())
}

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

#[test]
fn test_shl() -> Result<(), DeboaError> {
    let api = Deboa::new();
    let request = api << "https://httpbin.org/get";

    assert_eq!(request.url(), "https://httpbin.org/get");

    Ok(())
}
