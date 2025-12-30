use deboa::{Client, Result};
use deboa_macros::delete;

#[tokio::test]
async fn delete() -> Result<()> {
    let mut client = Client::default();
    let response = delete!("https://jsonplaceholder.typicode.com/posts/1", &mut client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}

#[tokio::test]
async fn delete_with_headers() -> Result<()> {
    let mut client = Client::default();
    let headers = vec![("User-Agent", "deboa")];
    let response = delete!("https://jsonplaceholder.typicode.com/posts/1", headers, &mut client);
    assert!(response
        .status()
        .is_success());
    Ok(())
}
