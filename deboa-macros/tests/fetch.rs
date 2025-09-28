use deboa::{errors::DeboaError, Deboa};
use deboa_extras::http::serde::json::JsonBody;
use deboa_macros::{fetch};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

#[tokio::test]
async fn test_fetch_str() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    let response = fetch!("https://jsonplaceholder.typicode.com/posts" -> JsonBody -> Vec<Post>, using &mut client);
    assert_eq!(response.len(), 100);
    Ok(())
}


#[tokio::test]
async fn test_fetch_ident() -> Result<(), DeboaError> {
    let mut client = Deboa::new();
    let url = "https://jsonplaceholder.typicode.com/posts";
    let response = fetch!(url -> JsonBody -> Vec<Post>, using &mut client);
    assert_eq!(response.len(), 100);
    Ok(())
}
