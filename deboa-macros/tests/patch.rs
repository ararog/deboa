use deboa::{Deboa, errors::DeboaError};

#[macro_use]
extern crate deboa_macros;

mod inner {

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
        patch(name="updatePost", path="/posts/<id:i32>", req_body=Post, format="json"),
      )
    )]
    pub struct PostService;
}

#[tokio::test]
async fn test_patch_by_id() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

    post_service
        .updatePost(
            1,
            inner::Post {
                title: "title".to_string(),
                body: "body".to_string(),
                user_id: 1,
            },
        )
        .await?;
    Ok(())
}
