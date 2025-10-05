use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::utils::setup_server;
use http::StatusCode;
use httpmock::{Method::POST, MockServer};
use serde::Serialize;
use vamo::{resource::AsPostRequest, Vamo};

#[derive(vamo_macros::Resource, Serialize)]
#[post("/users")]
#[put("/users/{}")]
#[patch("/users/{}")]
#[delete("/users/{}")]
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

    let user = User {
        id: 32,
        name: "User 1".to_string(),
    };

    let mut url = server.base_url();
    url.push_str("/api");
    let mut vamo = Vamo::new(url.to_string())?;
    let response = vamo.go(user.as_post_request()?).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}
