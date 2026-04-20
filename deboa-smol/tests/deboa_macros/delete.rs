use std::error::Error;

use deboa_macros::delete;
use deboa_smol::Client;

use macro_rules_attribute::apply;
use smol_macros::test;

#[apply(test!)]
async fn delete() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let response = delete!("https://jsonplaceholder.typicode.com/posts/1", &client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[apply(test!)]
async fn delete_with_headers() -> Result<(), Box<dyn Error>> {
    let client = Client::default();
    let headers = vec![("User-Agent", "deboa")];
    let response = delete!("https://jsonplaceholder.typicode.com/posts/1", headers, &client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}
