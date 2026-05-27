use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::get;
use deboa_tokio::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_get_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!(
        url => "https://jsonplaceholder.typicode.com/posts",
        client => &client
    );
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!(
        url => "https://jsonplaceholder.typicode.com/posts",
        headers => vec![("Content-Type", "application/json")],
        client => &client
    );
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!(
        url => "https://jsonplaceholder.typicode.com/posts",
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 100);
    Ok(())
}

#[tokio::test]
async fn test_get_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!(
        url => "https://jsonplaceholder.typicode.com/posts",
        headers => vec![("User-Agent", "deboa")],
        client => &client,
        res_body_ty => JsonBody,
        res_ty => Vec<Post>
    );
    assert_eq!(response.len(), 100);
    Ok(())
}
