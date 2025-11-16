use deboa::Result;
use deboa_extras::http::serde::json::JsonBody;
use serde::Deserialize;
use tokio::sync::Mutex;
use std::sync::Arc;
use vamo::Vamo;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let vamo = Arc::new(Mutex::new(Vamo::new("https://jsonplaceholder.typicode.com")?));
    let ids = vec![1, 16, 21, 26, 31, 36, 41, 46, 51, 56, 61, 66];
    let mut handles = vec![];
    for id in ids {
        let vamo_clone = Arc::clone(&vamo);
        handles.push(tokio::spawn(async move {
            let post = fetch_post(vamo_clone, id).await;
            if let Ok(post) = post {
                println!("Response: {post:#?}");
            } else {
                println!("Error: {}", post.unwrap_err());
            }
        }));
    }
    
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}

async fn fetch_post(vamo: Arc<Mutex<Vamo>>, id: u32) -> Result<Post> {
    let post: Post = vamo
        .lock()
        .await
        .get(format!("/posts/{id}").as_str())
        .send()
        .await?
        .body_as(JsonBody)
        .await?;
    Ok(post)
}
