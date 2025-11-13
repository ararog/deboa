use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{data::{JSON_STR_PATCH, JSON_STR_POST}, utils::{setup_server, setup_server_with_body}};
use http::{header, StatusCode};
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};
use serde::Serialize;

use crate::{resource::{Resource, ResourceMethod}, Vamo};

#[derive(Serialize)]
struct Post {
    id: u64,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<u64>,
}

impl Resource for Post {
    fn id(&self) -> String {
        self.id.to_string()
    }

    fn get_path(&self) -> &str {
        "/posts/:id"
    }

    fn post_path(&self) -> &str {
        "/posts"
    }

    fn delete_path(&self) -> &str {
        "/posts/:id"
    }

    fn put_path(&self) -> &str {
        "/posts/:id"
    }

    fn patch_path(&self) -> &str {
        "/posts/:id"
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
    let response = vamo.get("/posts").send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", PUT, StatusCode::OK);

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.put("/posts").send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<()> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/api/posts").body(
            "{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}",
        );
        then.status::<u16>(StatusCode::CREATED.into())
            .header(header::CONTENT_TYPE.as_str(), "application/json")
            .body("{}");
    });

    let post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.post("/posts").body_as(JsonBody, post)?.send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/posts/1", PATCH, StatusCode::OK);

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.patch("/posts/1").send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/posts/1", DELETE, StatusCode::NO_CONTENT);

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.delete("/posts/1").send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}

#[tokio::test]
async fn test_post_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server_with_body(&server, "/api/posts", POST, StatusCode::CREATED, JSON_STR_POST);

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.post_resource(&mut post)?.send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_put_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server_with_body(&server, "/api/posts/1", PUT, StatusCode::OK, JSON_STR_POST);

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.put_resource(&mut post)?.send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_patch_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server_with_body(&server, "/api/posts/1", PATCH, StatusCode::OK, JSON_STR_PATCH);

    let mut post = Post {
        id: 1,
        title: "Some other title".to_string(),
        body: None,
        user_id: None,
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo.patch_resource(&mut post)?.send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}
