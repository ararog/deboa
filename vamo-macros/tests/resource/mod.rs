use deboa::{client::serde::RequestBody, Result};
use deboa_extras::http::serde::json::JsonBody;
use deboa_tests::{
    server::{tcp::tokio::HttpServer, ServerConfig},
    utils::make_response,
};
use http::StatusCode;
use serde::Serialize;
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
async fn test_post_resource() -> Result<()> {
    let config: Option<ServerConfig> = Some(ServerConfig::new(
        Some("certs/server.cert".to_string()),
        Some("certs/server.key".to_string()),
    ));
    let mut server = HttpServer::new(config);
    #[allow(unused_must_use)]
    server
        .start(|req| {
            if req.method() == "POST" && req.uri().path() == "/api/users" {
                Ok(make_response(StatusCode::OK, b""))
            } else {
                Ok(make_response(StatusCode::NOT_FOUND, b"Not found"))
            }
        })
        .await;

    let mut user = User { id: 32, name: "User 1".to_string() };

    let mut url = server.base_url();
    url.push_str("/api");
    let mut vamo = Vamo::new(url.to_string())?;
    let response = vamo
        .create(&mut user)?
        .send()
        .await?;

    assert_eq!(response.status(), StatusCode::CREATED);

    server.stop().await;

    Ok(())
}
