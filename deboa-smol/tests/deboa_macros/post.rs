use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::post;
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

#[apply(test!)]
async fn test_only_post_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(data, "https://jsonplaceholder.typicode.com/posts", &client);
    assert_eq!(response.status(), 201);
    Ok(())
}

#[apply(test!)]
async fn test_only_post_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response = post!(data, "https://jsonplaceholder.typicode.com/posts", headers, &client);
    assert_eq!(response.status(), 201);
    Ok(())
}

#[apply(test!)]
async fn test_only_post() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = post!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts", &client);
    assert_eq!(response.status(), 201);
    Ok(())
}

#[apply(test!)]
async fn test_post_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let headers = vec![("Content-Type", "application/json")];
    let response =
        post!(data, JsonBody, "https://jsonplaceholder.typicode.com/posts", headers, &client);
    assert_eq!(response.status(), 201);
    Ok(())
}
