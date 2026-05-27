use std::error::Error;

use deboa_macros::submit;
use deboa_tokio::Client;
use http::Method;

#[tokio::test]
async fn test_submit_str_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
        url => "https://jsonplaceholder.typicode.com/posts",
        client => &client
    );
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[tokio::test]
async fn test_submit_str_method() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let headers = vec![("Content-Type", "application/x-www-form-urlencoded")];
    let response = submit!(
        method => Method::POST,
        data => "user=deboa",
        url => "https://jsonplaceholder.typicode.com/posts",
        headers => headers,
        client => &client
    );
    assert!(response
        .status()
        .is_success());
    Ok(())
}
