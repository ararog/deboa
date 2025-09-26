use deboa::errors::DeboaError;
use deboa_macros::bora;
use deboa_tests::utils::JSONPLACEHOLDER;
use vamo::Vamo;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
}

#[bora(
      api(
        get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
        get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
      )
    )]
pub struct PostService;

#[tokio::test]
async fn test_get_by_id() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    let post = post_service.get_by_id(1).await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
    Ok(())
}

#[tokio::test]
async fn test_get_all() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    let posts = post_service.get_all().await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 100);
    Ok(())
}

#[tokio::test]
async fn test_query_by_id() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    let posts = post_service.query_by_id(1).await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}

#[tokio::test]
async fn test_query_by_title() -> Result<(), DeboaError> {
    let client = Vamo::new(JSONPLACEHOLDER)?;

    let mut post_service = PostService::new(client);

    let posts = post_service.query_by_title(6, "dolorem eum magni eos aperiam quia").await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 1);
    Ok(())
}
