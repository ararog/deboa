use deboa::{errors::DeboaError, Deboa};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_get() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    let response = get!("https://jsonplaceholder.typicode.com/posts" => &mut client => JsonBody => Vec<Post>);
    assert_eq!(response.len(), 100);
    Ok(())
}
