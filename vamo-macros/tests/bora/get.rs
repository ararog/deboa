use deboa::{
    cert::{Certificate, ContentEncoding},
    Client as DeboaClient, Result,
};
use deboa_tests::{
    server::Server,
    utils::{make_response, tls_server_config, CA_CERT},
};

#[cfg(all(feature = "_tokio-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::tokio::HttpServer;

#[cfg(all(feature = "_smol-rt", any(feature = "_http1", feature = "_http2")))]
use deboa_tests::server::tcp::smol::HttpServer;

#[cfg(all(feature = "_tokio-rt", feature = "_http3"))]
use deboa_tests::server::udp::tokio::HttpServer;

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

async fn do_get_by_id() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| async move {
            if req.method() == "GET" && req.uri().path() == "/posts/1" {
                Ok(make_response(StatusCode::OK, b"{ \"id\": 1, \"title\": \"title\" }"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_get_by_id() -> Result<()> {
    do_get_by_id().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_get_by_id() -> Result<()> {
    do_get_by_id().await
}

async fn do_get_all() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| async move {
            if req.method() == "GET" && req.uri().path() == "/posts" {
                Ok(make_response(
                    StatusCode::OK,
                    b"[{ \"id\": 1, \"title\": \"title\" }, { \"id\": 2, \"title\": \"title\" }]",
                ))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 2);
    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_get_all() -> Result<()> {
    do_get_all().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_get_all() -> Result<()> {
    do_get_all().await
}

async fn do_query_by_id() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| async move {
            println!("{} {}", req.method(), req.uri());
            if req.method() == "GET"
                && req.uri().path() == "/posts"
                && req.uri().query() == Some("id=1")
            {
                Ok(make_response(StatusCode::OK, b"[{ \"id\": 1, \"title\": \"title\" }]"))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_query_by_id() -> Result<()> {
    do_query_by_id().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_query_by_id() -> Result<()> {
    do_query_by_id().await
}

async fn do_query_by_title() -> Result<()> {
    let mut server = HttpServer::new(tls_server_config());
    #[allow(unused_must_use)]
    server
        .start(|req| async move {
            if req.method() == "GET"
                && req.uri().path() == "/posts"
                && req.uri().query()
                    == Some("id=6&title=dolorem%20eum%20magni%20eos%20aperiam%20quia")
            {
                Ok(make_response(
                    StatusCode::OK,
                    b"[{ \"id\": 6, \"title\": \"dolorem eum magni eos aperiam quia\" }]",
                ))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
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

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

#[cfg(feature = "_tokio-rt")]
#[tokio::test]
async fn test_query_by_title() -> Result<()> {
    do_query_by_title().await
}

#[cfg(feature = "_smol-rt")]
#[apply(test!)]
async fn test_query_by_title() -> Result<()> {
    do_query_by_title().await
}
