use std::error::Error;

use crate::{
    resource::{Resource, ResourceMethod},
    Vamo,
};
use deboa::{url::IntoUrl, HttpClient, Result};
use deboa_extras::http::serde::json::JsonBody;
use http::{HeaderName, Method};
use serde::Serialize;

struct SuperClient {
    url: String,
}

impl Default for SuperClient {
    fn default() -> Self {
        Self { url: test_url(None) }
    }
}

impl HttpClient for SuperClient {
    async fn execute<R>(&self, _request: R) -> Result<deboa::response::DeboaResponse>
    where
        R: deboa::request::IntoRequest,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Serialize)]
struct User {
    id: i32,
    name: String,
}

impl Resource for User {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn name(&self) -> &str {
        "users"
    }

    fn body_type(&self) -> impl deboa::serde::RequestBody {
        JsonBody
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
fn test_load_resource() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    let mut user = User { id: 1, name: "John Doe".to_string() };
    vamo.load(&mut user)?;
    assert_eq!(vamo.path, "/users/1");
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

#[test]
fn test_header() -> std::result::Result<(), Box<dyn Error>> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    let header = "Content-Type".parse::<HeaderName>()?;
    vamo.header(header, "application/json");
    assert_eq!(
        vamo.headers
            .get("Content-Type")
            .unwrap()
            .to_str()
            .unwrap(),
        "application/json"
    );
    Ok(())
}

#[test]
fn test_basic_auth() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.basic_auth("username", "password");
    assert_eq!(
        vamo.headers
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        "Basic dXNlcm5hbWU6cGFzc3dvcmQ="
    );
    Ok(())
}

#[test]
fn test_jwt_auth() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.bearer_auth("token");
    assert_eq!(
        vamo.headers
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap(),
        "Bearer token"
    );
    Ok(())
}
