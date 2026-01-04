use deboa::{Client, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::patch;
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
async fn test_only_patch_minimal() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", &mut client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_only_patch_minimal_headers() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response =
        patch!(data, "https://jsonplaceholder.typicode.com/posts/1", headers, &mut client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", &mut client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_patch_with_headers() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response =
        patch!(data, "https://jsonplaceholder.typicode.com/posts/1", headers, &mut client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_request() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data,
        JsonBody,
        "https://jsonplaceholder.typicode.com/posts/1",
        headers,
        &mut client
    );
    assert_eq!(response.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_no_headers() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data,
        JsonBody,
        "https://jsonplaceholder.typicode.com/posts/1",
        &mut client,
        JsonBody,
        PostWithId
    );
    assert_eq!(response.id, 1);
    Ok(())
}

#[tokio::test]
async fn test_patch_with_json_body_response() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data,
        JsonBody,
        "https://jsonplaceholder.typicode.com/posts/1",
        headers,
        &mut client,
        JsonBody,
        Post
    );
    assert_eq!(response.id, 1);
    Ok(())
}
