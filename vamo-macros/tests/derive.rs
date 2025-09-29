use deboa::client::serde::RequestBody;
use deboa_extras::http::serde::json::JsonBody;
use vamo::resource::Resource;

#[derive(vamo_macros::Resource)]
#[post("/users")]
#[put("/users/{id}")]
#[patch("/users/{id}")]
#[delete("/users/{id}")]
#[body_type(JsonBody)]
pub struct User {
    id: String,
    name: String,
}

#[tokio::test]
async fn test_resource() -> () {
    let user = User {
        id: "1".to_string(),
        name: "John Doe".to_string(),
    };
    assert_eq!(user.post_path(), "/users");
    assert_eq!(user.put_path(), "/users/{id}");
    assert_eq!(user.patch_path(), "/users/{id}");
    assert_eq!(user.delete_path(), "/users/{id}");
}
