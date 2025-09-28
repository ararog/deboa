use bora::bora;
use deboa::errors::DeboaError;
use deboa_tests::utils::JSONPLACEHOLDER;
use serde::{Deserialize, Serialize};
use vamo::Vamo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[bora(
      api(
        post(name="create_post", path="/posts", req_body=Post, format="json"),
      )
    )]
pub struct PostService;

#[tokio::test]
async fn test_get_by_id() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    post_service
        .create_post(Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            user_id: 1,
        })
        .await?;
    Ok(())
}
