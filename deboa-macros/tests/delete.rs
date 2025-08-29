use deboa::{Deboa, DeboaError};

#[macro_use]
extern crate deboa_macros;

mod inner {

    #[bora(api(delete(name = "deletePost", path = "/posts/<id:i32>"),))]
    pub struct PostService;
}

#[tokio::test]
async fn test_delete_by_id() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let post_service = PostService::new(deboa);

    post_service.deletePost(1).await?;
    Ok(())
}
