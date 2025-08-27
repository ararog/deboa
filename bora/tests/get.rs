use deboa::Deboa;

#[macro_use]
extern crate bora;

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
async fn test_get_by_id() {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com");

    let post_service = PostService::new(deboa);

    let post = post_service.get_by_id(1).await.unwrap();

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
}

#[tokio::test]
async fn test_get_all() {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com");

    let post_service = PostService::new(deboa);

    let posts = post_service.get_all().await.unwrap();

    println!("posts: {posts:?}");

    assert_eq!(posts.len(), 100);
}
