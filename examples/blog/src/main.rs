use deboa::{Client, Result, async_trait};
use deboa_extras::http::serde::json::JsonBody;
use http::{HeaderValue, header::AUTHORIZATION};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use vamo::Vamo;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

struct AuthCatcher;

#[async_trait]
impl deboa::catcher::DeboaCatcher for AuthCatcher {
    async fn on_request(
        &self,
        request: &mut deboa::request::DeboaRequest,
    ) -> Result<Option<deboa::response::DeboaResponse>> {
        request
            .headers_mut()
            .insert(AUTHORIZATION, HeaderValue::from_str("Bearer token").unwrap());
        Ok(None)
    }

    async fn on_response(&self, _response: &mut deboa::response::DeboaResponse) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .catch(AuthCatcher)
        .build();
    let vamo = Arc::new(Mutex::new(Vamo::new("https://jsonplaceholder.typicode.com")?));
    vamo.lock()
        .await
        .client(client);

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
        .get(&format!("/posts/{id}"))
        .send()
        .await?
        .body_as(JsonBody)
        .await?;
    Ok(post)
}
