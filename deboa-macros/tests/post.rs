use deboa::{Deboa, errors::DeboaError};
use deboa_macros::bora;
use serde::{Deserialize, Serialize};

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
    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

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
