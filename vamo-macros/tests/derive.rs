use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::utils::setup_server;
use http::StatusCode;
use httpmock::{Method::POST, MockServer};
use serde::Serialize;
use vamo::{Vamo, resource::ResourceMethod};
use vamo_macros::Resource;

#[derive(Resource, Serialize)]
#[get("/users/:id")]
#[post("/users")]
#[put("/users/:id")]
#[patch("/users/:id")]
#[delete("/users/:id")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

#[tokio::test]
async fn test_post_resource() -> Result<()> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/api/users", POST, StatusCode::CREATED);

    let mut user = User {
        id: 32,
        name: "User 1".to_string(),
    };

    let mut url = server.base_url();
    url.push_str("/api");
    let mut vamo = Vamo::new(url.to_string())?;
    let response = vamo.post_resource(&mut user)?.send().await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}
