use crate::Deboa;
use crate::errors::DeboaError;

#[test]
fn test_set_retries() -> Result<(), DeboaError> {
    let mut api = Deboa::new();

    api.set_retries(5);

    assert_eq!(api.retries, 5);

    Ok(())
}

#[test]
fn test_set_connection_timeout() -> Result<(), DeboaError> {
    let mut api = Deboa::new();

    api.set_connection_timeout(5);

    assert_eq!(api.connection_timeout, 5);

    Ok(())
}

#[test]
fn test_set_request_timeout() -> Result<(), DeboaError> {
    let mut api = Deboa::new();

    api.set_request_timeout(5);

    assert_eq!(api.request_timeout, 5);

    Ok(())
}
