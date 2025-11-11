use deboa::{
    request::{get, FetchWith},
    Deboa, Result,
};
use deboa_extras::http::serde::json::JsonBody;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Deboa::new();

    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", 1);
    let response: Post = url
        .as_str()
        .fetch_with(&mut client)
        .await?
        .body_as(JsonBody)
        .await?;

    println!("post: {response:#?}");

    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
        .with(&mut client)
        .await?
        .body_as(JsonBody)
        .await?;

    println!("posts: {posts:#?}");

    Ok(())
}
