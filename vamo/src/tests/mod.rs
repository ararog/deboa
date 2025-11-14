use crate::Vamo;
use deboa::{url::IntoUrl, HttpVersion, Result};
use http::Method;

const JSONPLACEHOLDER: &str = "https://jsonplaceholder.typicode.com";

mod integrated;

#[test]
fn test_create_vamo() -> Result<()> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    assert_eq!(vamo.base_url, JSONPLACEHOLDER.into_url()?);
    Ok(())
}

#[test]
fn test_client() -> Result<()> {
    let vamo = Vamo::new(JSONPLACEHOLDER)?;
    assert_eq!(
        vamo.client
            .protocol(),
        &HttpVersion::Http1
    );
    Ok(())
}

#[test]
fn test_get() -> Result<()> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    vamo.get("/posts");
    assert_eq!(vamo.method, Method::GET);
    Ok(())
}

#[test]
fn test_post() -> Result<()> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    vamo.post("/posts");
    assert_eq!(vamo.method, Method::POST);
    Ok(())
}

#[test]
fn test_put() -> Result<()> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    vamo.put("/posts");
    assert_eq!(vamo.method, Method::PUT);
    Ok(())
}

#[test]
fn test_patch() -> Result<()> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    vamo.patch("/posts");
    assert_eq!(vamo.method, Method::PATCH);
    Ok(())
}

#[test]
fn test_delete() -> Result<()> {
    let mut vamo = Vamo::new(JSONPLACEHOLDER)?;
    vamo.delete("/posts");
    assert_eq!(vamo.method, Method::DELETE);
    Ok(())
}
