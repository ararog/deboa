use deboa::{
    request::{get, DeboaRequest, FetchWith},
    Client, Result,
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
    let mut client = Client::default();

    let response: Post = format!("https://jsonplaceholder.typicode.com/posts/{}", 1)
        .fetch_with(&mut client)
        .await?
        .body_as(JsonBody)
        .await?;

    println!("post: {response:#?}");

    let posts: Vec<Post> = get("https://jsonplaceholder.typicode.com/posts")?
        .send_with(&mut client)
        .await?
        .body_as(JsonBody)
        .await?;

    println!("posts: {posts:#?}");

    let request = r##"
    POST https://jsonplaceholder.typicode.com/posts
    Content-Type: application/json

    {
        "title": "foo",
        "body": "bar",
        "userId": 1
    }
    "##
    .parse::<DeboaRequest>()?;

    let response: Post = client
        .execute(request)
        .await?
        .body_as(JsonBody)
        .await?;

    println!("saved post: {response:#?}");

    Ok(())
}
