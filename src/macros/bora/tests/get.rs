#[macro_use]
extern crate bora;

use deboa::Deboa;

mod inner {

    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Post {
        pub id: u32,
        pub title: String
    }

    #[bora(GET "/posts/1" Post)]
    pub struct PostService;
}

#[tokio::test]
async fn main() {
    use inner::{PostService, Service};

    let deboa = Deboa::new("https://jsonplaceholder.typicode.com");

    let post_service = PostService::new(deboa);

    let post = post_service.get().await.unwrap();

    println!("id...: {}", post.id);
    println!("title: {}", post.title);

    assert_eq!(post.id, 1);
}
