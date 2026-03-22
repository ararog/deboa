use deboa::{
    cert::{Certificate, ContentEncoding},
    Client as DeboaClient,
};
use deboa_tests::{
    mock_response,
    utils::{start_mock_server, CA_CERT},
};

use http::StatusCode;
use vamo::Vamo;
use vamo_macros::bora;

use serde::Deserialize;

use crate::SKIP_CERT_VERIFICATION;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
      api(
        get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
        get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
      )
    )]
pub struct PostService;

async fn do_get_by_id() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts/1" {
            Ok(mock_response(StatusCode::OK, "{ \"id\": 1, \"title\": \"title\" }"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    let post = post_service
        .get_by_id(1)
        .await?;

    server
        .stop()
        .await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_get_by_id().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_get_by_id().await
}

async fn do_get_all() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET" && req.uri().path() == "/posts" {
            Ok(mock_response(
                StatusCode::OK,
                "[{ \"id\": 1, \"title\": \"title\" }, { \"id\": 2, \"title\": \"title\" }]",
            ))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    let posts = post_service
        .get_all()
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 2);
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_all() -> Result<(), Box<dyn std::error::Error>> {
    do_get_all().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_get_all() -> Result<(), Box<dyn std::error::Error>> {
    do_get_all().await
}

async fn do_query_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        println!("{} {}", req.method(), req.uri());
        if req.method() == "GET"
            && req.uri().path() == "/posts"
            && req.uri().query() == Some("id=1")
        {
            Ok(mock_response(StatusCode::OK, "[{ \"id\": 1, \"title\": \"title\" }]"))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    let posts = post_service
        .query_by_id(1)
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_query_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_query_by_id().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_query_by_id() -> Result<(), Box<dyn std::error::Error>> {
    do_query_by_id().await
}

async fn do_query_by_title() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = start_mock_server(|req| async move {
        if req.method() == "GET"
            && req.uri().path() == "/posts"
            && req.uri().query() == Some("id=6&title=dolorem%20eum%20magni%20eos%20aperiam%20quia")
        {
            Ok(mock_response(
                StatusCode::OK,
                "[{ \"id\": 6, \"title\": \"dolorem eum magni eos aperiam quia\" }]",
            ))
        } else {
            Ok(mock_response(StatusCode::NOT_FOUND, "Not found"))
        }
    })
    .await;

    let client = DeboaClient::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut vamo = Vamo::new(server.base_url())?;
    vamo.client(client);

    let mut post_service = PostService::new(vamo);

    let posts = post_service
        .query_by_title(6, "dolorem eum magni eos aperiam quia")
        .await?;

    server
        .stop()
        .await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_query_by_title() -> Result<(), Box<dyn std::error::Error>> {
    do_query_by_title().await
}

#[cfg(feature = "smol-rt")]
#[apply(test!)]
async fn test_query_by_title() -> Result<()> {
    do_query_by_title().await
}
