use deboa::errors::DeboaError;
use deboa_macros::bora;
use deboa_tests::utils::JSONPLACEHOLDER;
use vamo::Vamo;

#[bora(api(delete(name = "delete_post", path = "/posts/<id:i32>")))]
pub struct PostService;

#[tokio::test]
async fn test_delete_by_id() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    post_service.delete_post(1).await?;
    Ok(())
}
