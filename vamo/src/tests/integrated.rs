use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{
    data::{JSON_STR_PATCH, JSON_STR_POST},
    server::{tcp::tokio::HttpServer, ServerConfig},
    utils::make_response,
};
use http::{header, StatusCode};
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};
use serde::Serialize;

use crate::{
    resource::{Resource, ResourceMethod},
    Vamo,
};

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

    fn name(&self) -> &str {
        "posts"
    }

    fn body_type(&self) -> impl RequestBody {
        JsonBody
    }
}

#[tokio::test]
async fn test_get() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "GET" {
                match req.uri().path() {
                    "/posts" => Ok(make_response(StatusCode::OK, b"pong")),
                    "/posts/1" => Ok(make_response(StatusCode::OK, b"pong")),
                    _ => Ok(make_response(StatusCode::NOT_FOUND, b"Not found")),
                }
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut vamo = Vamo::new(
        server
            .base_url()
            .as_str(),
    )?;
    let response = vamo
        .get("/posts")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let response = vamo
        .get("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .text()
            .await?,
        "pong"
    );

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "GET" && req.uri().path() == "/posts" {
                Ok(make_response(StatusCode::OK, b"pong"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut vamo = Vamo::new(
        server
            .base_url()
            .as_str(),
    )?;
    let response = vamo
        .put("/posts")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<()> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/posts")
            .body("{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}");
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
    let response = vamo
        .post("/posts")
        .body_as(JsonBody, post)?
        .send()
        .await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "PATCH" && req.uri().path() == "/api/posts/1" {
                Ok(make_response(StatusCode::OK, b"pong"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .patch("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "DELETE" && req.uri().path() == "/api/posts/1" {
                Ok(make_response(StatusCode::NO_CONTENT, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .delete("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_post_resource() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/api/posts" {
                Ok(make_response(StatusCode::CREATED, JSON_STR_POST.as_bytes()))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .create(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_put_resource() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "PUT" && req.uri().path() == "/api/posts/1" {
                Ok(make_response(StatusCode::OK, JSON_STR_POST.as_bytes()))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .update(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_patch_resource() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "PATCH" && req.uri().path() == "/api/posts/1" {
                Ok(make_response(StatusCode::OK, JSON_STR_PATCH.as_bytes()))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .edit(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}

#[tokio::test]
async fn test_remove_resource() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "DELETE" && req.uri().path() == "/api/posts/1" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let mut vamo = Vamo::new(server.url("/api"))?;
    let response = vamo
        .remove(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

    Ok(())
}
