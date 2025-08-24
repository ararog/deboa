use deboa::Deboa;

#[macro_use]
extern crate bora;

//use deboa::Deboa;

mod inner {

    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Post {
        pub id: u32,
        pub title: String
    }

    #[bora(
      api(
        get(name="get_by_id", path="/posts/1", target=Post),
        get(name="query_by_id", path="/posts?<id:i32>", target=Post),
        get(name="get_all", path="/posts", target=Post),
      )
    )]
    pub struct PostService;
}

#[tokio::test]
async fn main() {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com");

    let post_service = PostService::new(deboa);

    let post = post_service.get_by_id().await.unwrap();

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
}
