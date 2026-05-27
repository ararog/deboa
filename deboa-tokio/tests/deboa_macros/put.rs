use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::put;
use deboa_tokio::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostWithId {
    pub id: u32,
}

#[tokio::test]
async fn test_only_put_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        client => &client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_only_put_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        client => &client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_put_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_put_with_json_body_request() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        req_body_ty => JsonBody,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        headers => headers,
        client => &client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_put_with_json_body_no_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = put!(
        data => data,
        req_body_ty => JsonBody,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        client => &client,
        res_body_ty => JsonBody,
        res_ty => PostWithId
    );
    assert_eq!(response.id, 1);
    Ok(())
}

#[tokio::test]
async fn test_put_with_json_body_response() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = put!(
        data => data,
        req_body_ty => JsonBody,
        url => "https://jsonplaceholder.typicode.com/posts/1",
        headers => headers,
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Post
    );
    assert_eq!(response.id, 1);
    Ok(())
}
