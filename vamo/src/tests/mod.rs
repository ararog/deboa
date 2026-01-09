use crate::Vamo;
use deboa::{url::IntoUrl, Result};
use deboa_tests::utils::TEST_HOST;
use http::Method;

mod integrated;

#[test]
fn test_create_vamo() -> Result<()> {
    let vamo = Vamo::new(TEST_HOST)?;
    assert_eq!(vamo.base_url, TEST_HOST.into_url()?);
    Ok(())
}

#[test]
fn test_get() -> Result<()> {
    let mut vamo = Vamo::new(TEST_HOST)?;
    vamo.get("/posts");
    assert_eq!(vamo.method, Method::GET);
    Ok(())
}

#[test]
fn test_post() -> Result<()> {
    let mut vamo = Vamo::new(TEST_HOST)?;
    vamo.post("/posts");
    assert_eq!(vamo.method, Method::POST);
    Ok(())
}

#[test]
fn test_put() -> Result<()> {
    let mut vamo = Vamo::new(TEST_HOST)?;
    vamo.put("/posts");
    assert_eq!(vamo.method, Method::PUT);
    Ok(())
}

#[test]
fn test_patch() -> Result<()> {
    let mut vamo = Vamo::new(TEST_HOST)?;
    vamo.patch("/posts");
    assert_eq!(vamo.method, Method::PATCH);
    Ok(())
}

#[test]
fn test_delete() -> Result<()> {
    let mut vamo = Vamo::new(TEST_HOST)?;
    vamo.delete("/posts");
    assert_eq!(vamo.method, Method::DELETE);
    Ok(())
}
