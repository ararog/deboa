use deboa_macros::bora;
use vamo::Vamo;

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
pub struct Post {
    pub id: u64,
    pub title: String,
    pub body: String,
}

#[bora(
  api(
    get(name = "get_posts", path = "/posts", res_body = Vec<Post>, format = "json"),
    get(name = "get_post", path = "/posts/<id:i32>", res_body = Post, format = "json"),
    post(name = "create_post", path = "/posts", req_body = Post, format = "json"),
    put(name = "update_post", path = "/posts/<id:i32>", req_body = Post, format = "json"),
    delete(name = "delete_post", path = "/posts/<id:i32>"),
  )
)]
pub struct PostService;
