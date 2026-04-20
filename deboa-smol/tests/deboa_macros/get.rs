use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::get;
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
async fn test_get_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!("https://jsonplaceholder.typicode.com/posts", &client);
    assert!(!response.is_empty());
    Ok(())
}

#[apply(test!)]
async fn test_get_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!(
        "https://jsonplaceholder.typicode.com/posts",
        vec![("Content-Type", "application/json")],
        &client
    );
    assert!(!response.is_empty());
    Ok(())
}

#[apply(test!)]
async fn test_get() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = get!("https://jsonplaceholder.typicode.com/posts", &client, JsonBody, Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}

#[apply(test!)]
async fn test_get_with_headers() -> Result<(), Box<dyn Error>> {
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
