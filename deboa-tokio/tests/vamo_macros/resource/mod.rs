use crate::common::helpers::{create_client, create_server};
use deboa::serde::RequestBody;
use deboa_extras::http::serde::json::JsonBody;
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use http::StatusCode;
use serde::Serialize;
use std::error::Error;
use vamo::{resource::ResourceMethod, Vamo};
use vamo_macros::Resource;

#[derive(Resource, Serialize)]
#[name("users")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

#[tokio::test]
async fn do_post_resource() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/api/users"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;

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

    server
        .stop()
        .await?;

    Ok(())
}
