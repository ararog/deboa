use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::utils::setup_server;
use http::StatusCode;
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};
use serde::Serialize;

use crate::{
    resource::{AsPatchRequest, AsPostRequest, AsPutRequest, Resource},
    Vamo,
};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

impl Resource for User {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn post_path(&self) -> &str {
        "/users"
    }

    fn delete_path(&self) -> &str {
        "/users/{}"
    }

    fn put_path(&self) -> &str {
        "/users/{}"
    }

    fn patch_path(&self) -> &str {
        "/users/{}"
    }

    fn body_type(&self) -> impl RequestBody {
        JsonBody
    }
}

#[tokio::test]
async fn test_get() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", GET, StatusCode::OK);

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.get("/posts")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", PUT, StatusCode::OK);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.put("/posts")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", POST, StatusCode::CREATED);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.post("/posts")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts/1", PATCH, StatusCode::OK);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.patch("/posts/1")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts/1", DELETE, StatusCode::NO_CONTENT);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.delete("/posts/1")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[tokio::test]
async fn test_post_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/users", POST, StatusCode::CREATED);

    let user = User {
        id: 1,
        name: "User 1".to_string(),
        email: "user1@example.com".to_string(),
    };

    let mut vamo = Vamo::new(format!("{}{}", server.base_url(), "/api"))?;
    let response = vamo.go(user.as_post_request()?).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_put_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/users/1", PUT, StatusCode::OK);

    let user = User {
        id: 1,
        name: "User 1".to_string(),
        email: "user1@example.com".to_string(),
    };

    let mut vamo = Vamo::new(format!("{}{}", server.base_url(), "/api"))?;
    let response = vamo.go(user.as_put_request()?).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_patch_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/users/1", PATCH, StatusCode::OK);

    let user = User {
        id: 1,
        name: "User 1".to_string(),
        email: "user1@example.com".to_string(),
    };

    let mut vamo = Vamo::new(format!("{}{}", server.base_url(), "/api"))?;
    let response = vamo.go(user.as_patch_request()?).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
