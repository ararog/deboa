use std::error::Error;

use deboa_macros::submit;
use deboa_smol::Client;
use http::Method;

use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn test_submit_str_minimal() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response =
        submit!(Method::POST, "user=deboa", "https://jsonplaceholder.typicode.com/posts", &client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[apply(test!)]
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
