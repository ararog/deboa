use crate::common::helpers::{create_client, create_server};
use deboa::request::post;
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use easyhttpmock_vetis_tokio::{
    matchers::{method, path},
    mock::{given, AsyncMatcherExt, Mock, StatusCodeExt},
};
use fory::{Fory, ForyStruct};
use http::StatusCode;
use std::error::Error;

const FORY_PERSON: [u8; 33] = [
    1, 255, 28, 0, 11, 160, 254, 175, 118, 89, 59, 92, 194, 1, 68, 9, 0, 196, 72, 21, 52, 12, 32,
    30, 34, 74, 111, 104, 110, 32, 68, 111, 101,
];

#[derive(ForyStruct, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::test]
async fn do_fory_post_request() -> Result<(), Box<dyn Error>> {
    let mock = Mock::of(
        given(method("POST").and(path("/posts"))).will_return(
            StatusCode::OK
                .respond()
                .with_body(&FORY_PERSON),
        ),
    );

    let mut server = create_server().await;
    server
        .register_mock(mock)
        .await?;
    let client = create_client();

    let mut fory = Fory::default();
    let result = fory.register::<Person>(1);
    assert!(result.is_ok());

    let person = Person { name: "John Doe".to_string(), age: 30 };
    let request = post(server.url("/posts"))?.body_as_fory(&fory, person)?;
    let response: Person = request
        .send_with(&client)
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
