use deboa::{errors::DeboaError, request::DeboaRequest, Deboa};
use deboa_extras::http::serde::json::JsonBody;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let client = Deboa::new();

    let posts: Vec<Post> = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts")
        .go(client)
        .await?
        .body_as(JsonBody)?;

    println!("posts: {posts:#?}");

    Ok(())
}
