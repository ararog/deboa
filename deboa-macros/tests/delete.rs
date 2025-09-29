use deboa::errors::DeboaError;
use deboa_macros::delete;

#[tokio::test]
async fn delete() -> Result<(), DeboaError> {
    let mut client = deboa::Deboa::new();
    let response = delete!("https://jsonplaceholder.typicode.com/posts/1" -> &mut client);
    assert!(response.status().is_success());
    Ok(())
}
