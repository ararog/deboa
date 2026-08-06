use deboa::{
    cert::{Certificate, Identity},
    conn::HttpConnectionPool,
    dns::DnsResolver,
    request::post,
    Client, InnerClient, TestResult,
};
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use easyhttpmock::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
    server::ServerAdapter,
    EasyHttpMock,
};
use fory::{Fory, ForyStruct};
use http::StatusCode;

const FORY_PERSON: [u8; 33] = [
    1, 255, 28, 0, 11, 160, 254, 175, 118, 89, 59, 92, 194, 1, 68, 9, 0, 196, 72, 21, 52, 12, 32,
    30, 34, 74, 111, 104, 110, 32, 68, 111, 101,
];

#[derive(ForyStruct, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

pub async fn test_fory_post_request<S, I, C, P, R>(
    client: &Client<InnerClient<I, C, P, R>>,
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
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(&FORY_PERSON),
        ),
    );

    server
        .register_mock(mock)
        .await?;

    let mut fory = Fory::default();
    let result = fory.register::<Person>(1);
    assert!(result.is_ok());

    let person = Person { name: "John Doe".to_string(), age: 30 };
    let request = post(server.url("/posts"))?
        .version(protocol_version)
        .body_as_fory(&fory, person)?;
    let response: Person = request
        .send_with(client)
        .await?
        .body_as_fory(&fory)
        .await?;

    assert_eq!(response.name, "John Doe");
    assert_eq!(response.age, 30);

    server
        .stop()
        .await?;

    Ok(())
}
