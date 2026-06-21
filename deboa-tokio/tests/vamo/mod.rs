use std::error::Error;

use deboa::serde::RequestBody;
use deboa_extras::http::serde::json::JsonBody;
use deboa_tokio::Client;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use serde::Serialize;
use vamo::{
    resource::{Resource, ResourceMethod},
    Vamo,
};

use crate::common::{
    data::{JSON_PATCH, JSON_POST},
    helpers::{create_client, start_mock_server},
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
async fn test_get() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "GET"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"pong"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(
        server
            .base_url()
            .as_str(),
    )?;
    vamo.client(client);
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

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PUT"
            .has()
            .path("/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"pong"),
            ),
    );

    let server = start_mock_server(mock).await;

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(
        server
            .base_url()
            .as_str(),
    )?;
    vamo.client(client);
    let response = vamo
        .put("/posts")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .with_body(
                        b"{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}",
                    ),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .post("/posts")
        .body_as(JsonBody, post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(b"pong"),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .patch("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "DELETE"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::NO_CONTENT
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .delete("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_post_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/api/posts")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .with_body(JSON_POST),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .create(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_put_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PUT"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(JSON_PATCH),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .update(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_patch_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "PATCH"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(JSON_PATCH),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .edit(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server
        .assert()
        .await;

    Ok(())
}

#[tokio::test]
async fn test_remove_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "DELETE"
            .has()
            .path("/api/posts/1")
            .will_return(
                StatusCode::OK
                    .respond()
                    .no_body(),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let client = create_client();

    let mut vamo = Vamo::<Client>::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .remove(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server
        .assert()
        .await;

    Ok(())
}
