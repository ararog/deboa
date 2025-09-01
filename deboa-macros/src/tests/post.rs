use deboa::{Deboa, errors::DeboaError};

#[macro_use]
extern crate deboa_macros;

mod inner {

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
        post(name="createPost", path="/posts", req_body=Post, format="json"),
      )
    )]
    pub struct PostService;
}

#[tokio::test]
async fn test_get_by_id() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

    post_service
        .createPost(inner::Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
            user_id: 1,
        })
        .await?;
    Ok(())
}
