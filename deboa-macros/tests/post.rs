use deboa::{errors::DeboaError, Deboa};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::post;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_post() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    let data: Post = Post {
        id: 1,
        title: "title".to_string(),
        body: "body".to_string(),
    };
    let response = post!(data -> JsonBody -> "https://jsonplaceholder.typicode.com/posts" using &mut client);
    assert_eq!(response.status(), 201);
    Ok(())
}
