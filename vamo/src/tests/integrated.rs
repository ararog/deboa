use deboa::{cert::Certificate, client::serde::RequestBody, Client, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{
    data::{JSON_STR_PATCH, JSON_STR_POST},
    utils::{make_response, tls_server_config, CA_CERT},
};
use http::StatusCode;
use serde::Serialize;

#[cfg(all(feature = "_tokio-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "_smol-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "_tokio-rt", feature = "_http3"))]
use deboa_tests::server::udp::tokio::HttpServer;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
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

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "PUT" && req.uri().path() == "/posts" {
                Ok(make_response(StatusCode::OK, b"pong"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
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
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/api/posts" {
                Ok(make_response(
                    StatusCode::CREATED,
                    b"{\"id\":1,\"title\":\"Some title\",\"body\":\"Some body\",\"user_id\":1}",
                ))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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
        .certificate(Certificate::from_slice(CA_CERT))
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

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .patch("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .delete("/posts/1")
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .create(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .update(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .edit(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

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
    let mut server = HttpServer::new(tls_server_config());
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

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.url("/api"))?;
    vamo.client(client);
    let response = vamo
        .remove(&mut post)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    server.stop().await;

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
