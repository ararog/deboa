use deboa::DeboaError;

#[derive(Debug, serde::Deserialize)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[tokio::main]

async fn main() -> Result<(), DeboaError> {
    use deboa::Deboa;

    let api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    let posts: Vec<Post> = api.get("/posts").await?.json::<Vec<Post>>().await?;

    println!("posts: {posts:#?}");

    Ok(())
}
