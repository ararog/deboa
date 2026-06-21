use std::error::Error;

use deboa::serde::RequestBody;
use deboa_extras::http::serde::json::JsonBody;
use easyhttpmock_vetis_tokio::mock::{MethodExt, Mock, StatusCodeExt};
use http::StatusCode;
use serde::Serialize;
use vamo::{resource::ResourceMethod, Vamo};
use vamo_macros::Resource;

use crate::common::helpers::{create_client, start_mock_server};

#[derive(Resource, Serialize)]
#[name("users")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

async fn do_post_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        "POST"
            .has()
            .path("/api/users")
            .will_return(
                StatusCode::CREATED
                    .respond()
                    .no_body(),
            ),
    );

    let server = start_mock_server(mock).await;

    let mut user = User { id: 32, name: "User 1".to_string() };

    let mut url = server.base_url();
    url.push_str("/api");

    let client = create_client();

    let mut vamo = Vamo::new(url.to_string())?;
    vamo.client(client);
    let response = vamo
        .create(&mut user)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_post_resource() -> Result<(), Box<dyn Error>> {
    do_post_resource().await
}
