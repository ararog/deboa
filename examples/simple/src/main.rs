use deboa::errors::DeboaError;
use deboa_extras::http::serde::json::JsonBody;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[tokio::main]

async fn main() -> Result<(), DeboaError> {
    use deboa::Deboa;

    let mut api = Deboa::new("https://jsonplaceholder.typicode.com").unwrap();

    let posts: Vec<Post> = api.get("/posts/1").await?.body_as(JsonBody)?;

    println!("posts: {posts:#?}");

    Ok(())
}
