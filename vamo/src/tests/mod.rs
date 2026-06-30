use crate::{
    resource::{Resource, ResourceMethod},
    Vamo,
};
use caramelo::{
    expect,
    matchers::{eq, header_value},
};
use deboa::{url::IntoUrl, HttpClient, Result};
use deboa_extras::serde::json::JsonBody;
use http::{header, HeaderName, Method};
use serde::Serialize;
use std::error::Error;

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
    expect(vamo.base_url).to_be(eq(test_url(None).into_url()?));
    Ok(())
}

#[test]
fn test_load_resource() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    let mut user = User { id: 1, name: "John Doe".to_string() };
    vamo.load(&mut user)?;
    expect(vamo.path).to_be(eq("/users/1"));
    Ok(())
}

#[test]
fn test_get() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.get("/posts");
    expect(vamo.method).to_be(eq(Method::GET));
    Ok(())
}

#[test]
fn test_post() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.post("/posts");
    expect(vamo.method).to_be(eq(Method::POST));
    Ok(())
}

#[test]
fn test_put() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.put("/posts");
    expect(vamo.method).to_be(eq(Method::PUT));
    Ok(())
}

#[test]
fn test_patch() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.patch("/posts");
    expect(vamo.method).to_be(eq(Method::PATCH));
    Ok(())
}

#[test]
fn test_delete() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.delete("/posts");
    expect(vamo.method).to_be(eq(Method::DELETE));
    Ok(())
}

#[test]
fn test_header() -> std::result::Result<(), Box<dyn Error>> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    let header = "Content-Type".parse::<HeaderName>()?;
    vamo.header(header, "application/json");
    expect(vamo.headers).to_have(header_value("Content-Type", r"application/json"));
    Ok(())
}

#[test]
fn test_basic_auth() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.basic_auth("username", "password");
    expect(vamo.headers).to_have(header_value("Authorization", "Basic dXNlcm5hbWU6cGFzc3dvcmQ="));
    Ok(())
}

#[test]
fn test_jwt_auth() -> Result<()> {
    let mut vamo = Vamo::<SuperClient>::new(test_url(None))?;
    vamo.bearer_auth("token");
    expect(vamo.headers).to_have(header_value(header::AUTHORIZATION, "Bearer token"));
    Ok(())
}
