use std::error::Error;

use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::fetch;
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
async fn test_fetch_str_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = fetch!("https://jsonplaceholder.typicode.com/posts", &client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[apply(test!)]
async fn test_fetch_str_minimal_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!("https://jsonplaceholder.typicode.com/posts", headers, &client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[apply(test!)]
async fn test_fetch_str() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response =
        fetch!("https://jsonplaceholder.typicode.com/posts", &client, JsonBody, Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}

#[apply(test!)]
async fn test_fetch_ident() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let url = "https://jsonplaceholder.typicode.com/posts";
    let response = fetch!(url, &client, JsonBody, Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}

#[apply(test!)]
async fn test_fetch_ident_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let url = "https://jsonplaceholder.typicode.com/posts";
    let headers = vec![("User-Agent", "deboa")];
    let response = fetch!(url, headers, &client, JsonBody, Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}
