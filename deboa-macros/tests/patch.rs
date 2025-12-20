use deboa::{Client, Result};
use deboa_macros::patch;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_post() -> Result<()> {
    let mut client = Client::default();
    let data: Post = Post { id: 1, title: "title".to_string(), body: "body".to_string() };
    let response = patch!(data, "https://jsonplaceholder.typicode.com/posts/1", &mut client);
    assert_eq!(response.status(), 200);
    Ok(())
}
