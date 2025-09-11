use bora::Bora;
use deboa::errors::DeboaError;
use deboa_macros::bora;

#[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
pub struct PostService;

#[tokio::test]
async fn test_delete_by_id() -> Result<(), DeboaError> {
    let client = Bora::new("https://jsonplaceholder.typicode.com");

    let mut post_service = PostService::new(client);

    post_service.delete_post(1).await?;
    Ok(())
}
