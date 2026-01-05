use deboa::{Client, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_get_minimal() -> Result<()> {
    let client = Client::default();
    let response = get!("https://jsonplaceholder.typicode.com/posts", &client);
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get_minimal_headers() -> Result<()> {
    let client = Client::default();
    let response = get!(
        "https://jsonplaceholder.typicode.com/posts",
        vec![("Content-Type", "application/json")],
        &client
    );
    assert!(!response.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_get() -> Result<()> {
    let client = Client::default();
    let response = get!("https://jsonplaceholder.typicode.com/posts", &client, JsonBody, Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}

#[tokio::test]
async fn test_get_with_headers() -> Result<()> {
    let client = Client::default();
    let response = get!(
        "https://jsonplaceholder.typicode.com/posts",
        vec![("User-Agent", "deboa")],
        &client,
        JsonBody,
        Vec<Post>
    );
    assert_eq!(response.len(), 100);
    Ok(())
}
