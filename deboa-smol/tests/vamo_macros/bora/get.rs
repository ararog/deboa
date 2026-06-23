use std::error::Error;

use easyhttpmock_vetis_smol::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use macro_rules_attribute::apply;
use serde::Deserialize;
use smol_macros::test;
use vamo::Vamo;
use vamo_macros::bora;

use crate::common::helpers::{create_client, create_server};

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

async fn do_get_by_id() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts/1"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"{ \"id\": 1, \"title\": \"title\" }"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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

#[apply(test!)]
async fn test_get_by_id() -> Result<(), Box<dyn Error>> {
    do_get_by_id().await
}

async fn do_get_all() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(
                    b"[{ \"id\": 1, \"title\": \"title\" }, { \"id\": 2, \"title\": \"title\" }]",
                ),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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

#[apply(test!)]
async fn test_get_all() -> Result<(), Box<dyn Error>> {
    do_get_all().await
}

async fn do_query_by_id() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{ \"id\": 1, \"title\": \"title\" }]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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

#[apply(test!)]
async fn test_query_by_id() -> Result<(), Box<dyn Error>> {
    do_query_by_id().await
}

async fn do_query_by_title() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("GET").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(b"[{ \"id\": 6, \"title\": \"dolorem eum magni eos aperiam quia\" }]"),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

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

#[apply(test!)]
async fn test_query_by_title() -> Result<(), Box<dyn Error>> {
    do_query_by_title().await
}
