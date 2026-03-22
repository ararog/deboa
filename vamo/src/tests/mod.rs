use crate::Vamo;
use deboa::{url::IntoUrl, Result};
use deboa_tests::utils::test_url;
use http::Method;

mod integrated;

pub(crate) const SKIP_CERT_VERIFICATION: bool =
    cfg!(any(feature = "tokio-native-tls", feature = "smol-native-tls"));

#[test]
fn test_create_vamo() -> Result<()> {
    let vamo = Vamo::new(test_url(None))?;
    assert_eq!(vamo.base_url, test_url(None).into_url()?);
    Ok(())
}

#[test]
fn test_get() -> Result<()> {
    let mut vamo = Vamo::new(test_url(None))?;
    vamo.get("/posts");
    assert_eq!(vamo.method, Method::GET);
    Ok(())
}

#[test]
fn test_post() -> Result<()> {
    let mut vamo = Vamo::new(test_url(None))?;
    vamo.post("/posts");
    assert_eq!(vamo.method, Method::POST);
    Ok(())
}

#[test]
fn test_put() -> Result<()> {
    let mut vamo = Vamo::new(test_url(None))?;
    vamo.put("/posts");
    assert_eq!(vamo.method, Method::PUT);
    Ok(())
}

#[test]
fn test_patch() -> Result<()> {
    let mut vamo = Vamo::new(test_url(None))?;
    vamo.patch("/posts");
    assert_eq!(vamo.method, Method::PATCH);
    Ok(())
}

#[test]
fn test_delete() -> Result<()> {
    let mut vamo = Vamo::new(test_url(None))?;
    vamo.delete("/posts");
    assert_eq!(vamo.method, Method::DELETE);
    Ok(())
}
