use deboa::errors::DeboaError;
use vamo::Vamo;

use crate::post_service::{Post, PostService};

mod post_service;

#[tokio::main]
async fn main() -> Result<(), DeboaError> {
    let client = Vamo::new("https://jsonplaceholder.typicode.com")?;
    let mut post_service = PostService::new(client);

    println!("Listing posts...");
    let posts: Vec<Post> = post_service.get_posts().await.unwrap();
    println!("Posts: {posts:#?}\n");

    println!("Listing post with id 1...");
    let post = post_service.get_post(1).await.unwrap();
    println!("Post: {post:#?}\n");

    println!("Creating post...");
    let result = post_service
        .create_post(Post {
            id: 1,
            title: "title".to_string(),
            body: "body".to_string(),
        })
        .await;

    if result.is_err() {
        println!("Error creating post: {}\n", result.err().unwrap());
    }

    println!("Post successfully created\n");

    println!("Updating post with id 1...");
    let result = post_service
        .update_post(
            1,
            Post {
                id: 1,
                title: "title".to_string(),
                body: "body".to_string(),
            },
        )
        .await;

    if result.is_err() {
        println!("Error updating post: {}\n", result.err().unwrap());
    }

    println!("Post successfully updated\n");

    println!("Deleting post with id 1...");
    let result = post_service.delete_post(1).await;

    if result.is_err() {
        println!("Error deleting post: {}\n", result.err().unwrap());
    }

    println!("Post successfully deleted\n");

    Ok(())
}
