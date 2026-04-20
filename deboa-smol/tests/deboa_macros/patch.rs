use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::patch;
use deboa_smol::Client;
use serde::{Deserialize, Serialize};

use macro_rules_attribute::apply;
use smol_macros::test;

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

#[apply(test!)]
async fn test_only_patch_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", &client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[apply(test!)]
async fn test_only_patch_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", headers, &client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[apply(test!)]
async fn test_patch() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", &client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[apply(test!)]
async fn test_patch_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", headers, &client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[apply(test!)]
async fn test_patch_with_json_body_request() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response =
        patch!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts/1", headers, &client);
    assert_eq!(response.status(), 200);
    Ok(())
}

#[apply(test!)]
async fn test_patch_with_json_body_no_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(
        data,
        JsonBody,
        "https://jsonplaceholder.typicode.com/posts/1",
        &client,
        JsonBody,
        PostWithId
    );
    assert_eq!(response.id, 1);
    Ok(())
}

#[apply(test!)]
async fn test_patch_with_json_body_response() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = patch!(
        data,
        JsonBody,
        "https://jsonplaceholder.typicode.com/posts/1",
        headers,
        &client,
        JsonBody,
        Post
    );
    assert_eq!(response.id, 1);
    Ok(())
}
