use crate::Vamo;
use deboa::{HttpVersion, errors::DeboaError, request::IntoUrl};
use http::Method;

const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";

mod integrated;

#[test]
fn test_create_vamo() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;

    assert_eq!(vamo.base_url, JSONPLACEHOLDER.into_url()?);

    Ok(())
}

#[test]
fn test_client() -> Result<(), DeboaError> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    assert_eq!(vamo.client().protocol(), &HttpVersion::Http1);

    Ok(())
}

#[test]
fn test_get() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    let response = vamo.get("/posts")?.build().unwrap();
    assert_eq!(response.method(), Method::GET);

    Ok(())
}

#[test]
fn test_post() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    let response = vamo.post("/posts")?.build().unwrap();
    assert_eq!(response.method(), Method::POST);

    Ok(())
}

#[test]
fn test_put() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    let response = vamo.put("/posts")?.build().unwrap();
    assert_eq!(response.method(), Method::PUT);

    Ok(())
}

#[test]
fn test_patch() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    let response = vamo.patch("/posts")?.build().unwrap();
    assert_eq!(response.method(), Method::PATCH);

    Ok(())
}

#[test]
fn test_delete() -> Result<(), DeboaError> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    let response = vamo.delete("/posts")?.build().unwrap();
    assert_eq!(response.method(), Method::DELETE);

    Ok(())
}
