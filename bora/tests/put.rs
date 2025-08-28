use deboa::{Deboa, DeboaError};

#[macro_use]
extern crate bora;

mod inner {

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Post {
        pub title: String,
        pub body: String,
        #[serde(rename = "userId")]
        pub user_id: u32,
    }

    #[bora(
      api(
        put(name="patchPost", path="/posts/<id:i32>", req_body=Post, format="json"),
      )
    )]
    pub struct PostService;
}

#[tokio::test]
async fn test_put_by_id() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

    post_service
        .patchPost(
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
