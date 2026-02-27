use deboa::{
    cert::{Certificate, ContentEncoding},
    client::serde::RequestBody,
    Client, Result,
};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{
    data::{JSON_STR_PATCH, JSON_STR_POST},
    mock_response,
    utils::{start_mock_server, CA_CERT},
};
use http::StatusCode;
use serde::Serialize;

use crate::{
    resource::{Resource, ResourceMethod},
    tests::SKIP_CERT_VERIFICATION,
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

async fn do_get() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" {
            match req.uri().path() {
                "/posts" => Ok(mock_response(StatusCode::OK, "pong")),
                "/posts/1" => Ok(mock_response(StatusCode::OK, "pong")),
                _ => Ok(mock_response(StatusCode::NOT_FOUND, "Not found")),
            }
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(
        server
            .base_url()
            .as_str(),
    )?;
    vamo.client(client);
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

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_get() -> Result<()> {
    do_get().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_get() -> Result<()> {
    do_get().await
}

#[cfg(feature = "_tokio-rt")]
async fn do_put() -> Result<()> {
    let server = start_mock_server(|req| async move {
        if req.method() == "PUT" && req.uri().path() == "/posts" {
            Ok(mock_response(StatusCode::OK, "pong"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(
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

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_put() -> Result<()> {
    do_put().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_put() -> Result<()> {
    do_put().await
}

async fn do_post() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/api/posts" {
            Ok(mock_response(
                StatusCode::CREATED,
                "{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}",
            ))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .post("/posts")
        .body_as(JsonBody, post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_post() -> Result<()> {
    do_post().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_post() -> Result<()> {
    do_post().await
}

async fn do_patch() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PATCH" && req.uri().path() == "/api/posts/1" {
            Ok(mock_response(StatusCode::OK, "pong"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .patch("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_patch() -> Result<()> {
    do_patch().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_patch() -> Result<()> {
    do_patch().await
}

async fn do_delete() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "DELETE" && req.uri().path() == "/api/posts/1" {
            Ok(mock_response(StatusCode::NO_CONTENT, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .delete("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_delete() -> Result<()> {
    do_delete().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_delete() -> Result<()> {
    do_delete().await
}

async fn do_post_resource() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/api/posts" {
            Ok(mock_response(StatusCode::CREATED, JSON_STR_POST))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .create(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_post_resource() -> Result<()> {
    do_post_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_post_resource() -> Result<()> {
    do_post_resource().await
}

async fn do_put_resource() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PUT" && req.uri().path() == "/api/posts/1" {
            Ok(mock_response(StatusCode::OK, JSON_STR_POST))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let mut post = Post {
        id: 1,
        title: "Some title".to_string(),
        body: Some("Some body".to_string()),
        user_id: Some(1),
    };

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .update(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_put_resource() -> Result<()> {
    do_put_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_put_resource() -> Result<()> {
    do_put_resource().await
}

async fn do_patch_resource() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "PATCH" && req.uri().path() == "/api/posts/1" {
            Ok(mock_response(StatusCode::OK, JSON_STR_PATCH))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .edit(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_patch_resource() -> Result<()> {
    do_patch_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_patch_resource() -> Result<()> {
    do_patch_resource().await
}

async fn do_remove_resource() -> Result<()> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "DELETE" && req.uri().path() == "/api/posts/1" {
            Ok(mock_response(StatusCode::OK, ""))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let mut post = Post { id: 1, title: "Some other title".to_string(), body: None, user_id: None };

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .remove(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let _ = server.stop().await;

    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_remove_resource() -> Result<()> {
    do_remove_resource().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_remove_resource() -> Result<()> {
    do_remove_resource().await
}
