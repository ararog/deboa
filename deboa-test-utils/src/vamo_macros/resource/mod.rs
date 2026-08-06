use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    serde::RequestBody,
    url::IntoUrl as _,
    Client, InnerClient, TestResult,
};
use deboa_extras::serde::json::JsonBody;
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
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

pub async fn test_post_resource<S, I, C, P, R>(
    client: Client<InnerClient<I, C, P, R>>,
    server: &mut EasyHttpMock<S>,
    protocol_version: http::Version,
) -> TestResult<()>
where
    I: Identity + Send + Clone,
    C: Certificate + Send + Clone,
    P: HttpConnectionPool<Identity = I, Certificate = C> + Send,
    R: DnsResolver + Send,
    S: ServerAdapter,
{
    let mock = Mock::of(
        given(method("POST").and(path("/api/users"))).will_return(
            StatusCode::CREATED
                .respond()
                .no_body(),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut user = User { id: 32, name: "User 1".to_string() };
    let base_url = server
        .base_url()
        .into_url()?
        .join("/api")?;
    let mut vamo = Vamo::from_client(client)?
        .base_url(base_url)
        .version(protocol_version);
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
