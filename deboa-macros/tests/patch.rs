use bora::Bora;
use deboa::errors::DeboaError;
use deboa_macros::bora;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[bora(
      api(
        patch(name="patch_post", path="/posts/<id:i32>", req_body=Post, format="json"),
      )
    )]
pub struct PostService;

#[tokio::test]
async fn test_patch_by_id() -> Result<(), DeboaError> {
    let client = Bora::new("https://jsonplaceholder.typicode.com");

    let mut post_service = PostService::new(client);

    post_service
        .patch_post(
            1,
            Post {
                title: "title".to_string(),
                body: "body".to_string(),
                user_id: 1,
            },
        )
        .await?;
    Ok(())
}
