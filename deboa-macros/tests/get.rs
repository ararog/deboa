use deboa::{Deboa, errors::DeboaError};

#[macro_use]
extern crate deboa_macros;

mod inner {

    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Post {
        pub id: u32,
        pub title: String,
    }

    #[bora(
      api(
        get(name="get_by_id", path="/posts/<id:i32>", res_body=Post),
        get(name="query_by_id", path="/posts?<id:i32>", res_body=Post),
        get(name="get_all", path="/posts", res_body=Vec<Post>),
      )
    )]
    pub struct PostService;
}

#[tokio::test]
async fn test_get_by_id() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

    let post = post_service.get_by_id(1).await?;

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
    Ok(())
}

#[tokio::test]
async fn test_get_all() -> Result<(), DeboaError> {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com")?;

    let mut post_service = PostService::new(deboa);

    let posts = post_service.get_all().await?;

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 100);
    Ok(())
}
