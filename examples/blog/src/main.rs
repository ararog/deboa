use deboa::errors::DeboaError;
use deboa_extras::http::serde::json::JsonBody;
use serde::Deserialize;
use vamo::Vamo;

#[derive(Debug, Deserialize)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let posts = fetch_posts().await;
    if let Ok(posts) = posts {
        println!("Response: {posts:#?}");
    } else {
        println!("Error: {}", posts.unwrap_err());
    }
}

async fn fetch_posts() -> Result<Vec<Post>, DeboaError> {
    let mut vamo = Vamo::new("https://jsonplaceholder.typicode.com")?;
    let posts: Vec<Post> = vamo.get("/posts")?.go(vamo.client()).await?.body_as(JsonBody)?;
    Ok(posts)
}
