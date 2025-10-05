use deboa::Result;
use deboa_extras::http::serde::json::JsonBody;
use serde::Deserialize;
use vamo::Vamo;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

async fn fetch_posts() -> Result<Vec<Post>> {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com")?;
    let posts: Vec<Post> = vamo.get("/posts")?.go(vamo).await?.body_as(JsonBody)?;
    Ok(posts)
}
