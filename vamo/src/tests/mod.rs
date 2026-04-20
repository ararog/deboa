use crate::Vamo;
use deboa::{url::IntoUrl, HttpClient, Result};
use http::Method;

struct SuperClient {
    url: String,
}

impl Default for SuperClient {
    fn default() -> Self {
        Self { url: test_url(None) }
    }
}

impl HttpClient for SuperClient {
    async fn execute<R>(&self, request: R) -> Result<deboa::response::DeboaResponse>
    where
        R: deboa::request::IntoRequest,
    {
        todo!()
    }
}

const TEST_URL: &str = "https://localhost";

pub fn test_url(port: Option<u16>) -> String {
    if let Some(port) = port {
        format!("{}:{}", TEST_URL, port)
    } else {
        TEST_URL.to_string()
    }
}

#[test]
fn test_create_vamo() -> Result<()> {
    let vamo = Vamo::<SuperClient>::new(test_url(None))?;
    assert_eq!(vamo.base_url, test_url(None).into_url()?);
    Ok(())
}

#[test]
fn test_get() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.get("/posts");
    assert_eq!(vamo.method, Method::GET);
    Ok(())
}

#[test]
fn test_post() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.post("/posts");
    assert_eq!(vamo.method, Method::POST);
    Ok(())
}

#[test]
fn test_put() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.put("/posts");
    assert_eq!(vamo.method, Method::PUT);
    Ok(())
}

#[test]
fn test_patch() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.patch("/posts");
    assert_eq!(vamo.method, Method::PATCH);
    Ok(())
}

#[test]
fn test_delete() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.delete("/posts");
    assert_eq!(vamo.method, Method::DELETE);
    Ok(())
}
