use std::error::Error;

use deboa_macros::submit;
use deboa_tokio::Client;
use http::Method;

#[tokio::test]
async fn test_submit_str_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response =
        submit!(Method::POST, "user=deboa", "https://jsonplaceholder.typicode.com/posts", &client);
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
        Method::POST,
        "user=deboa",
        "https://jsonplaceholder.typicode.com/posts",
        headers,
        &client
    );
    assert!(response
        .status()
        .is_success());
    Ok(())
}
