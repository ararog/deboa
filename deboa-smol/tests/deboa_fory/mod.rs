use std::error::Error;

use deboa::request::post;
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use fory::{Fory, ForyStruct};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;

use crate::common::helpers::{create_client, start_mock_server};

const FORY_PERSON: [u8; 33] = [
    1, 255, 28, 0, 11, 160, 254, 175, 118, 89, 59, 92, 194, 1, 68, 9, 0, 196, 72, 21, 52, 12, 32,
    30, 34, 74, 111, 104, 110, 32, 68, 111, 101,
];

#[derive(ForyStruct, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[apply(test!)]
async fn do_fory_post_request() -> Result<(), Box<dyn Error>> {
    let path = "/posts";

    let mock = Mock::of(
        "POST"
            .has()
            .path("/posts")
            .will_return(
                StatusCode::OK
                    .respond()
                    .with_body(&FORY_PERSON),
            ),
    );

    let mut server = start_mock_server(mock).await;

    let client = create_client();

    let mut fory = Fory::default();
    let result = fory.register::<Person>(1);
    assert!(result.is_ok());

    let person = Person { name: "John Doe".to_string(), age: 30 };

    let request = post(server.url(path))?.body_as_fory(&fory, person)?;

    let response: Person = request
        .send_with(&client)
        .await?
        .body_as_fory(&fory)
        .await?;

    assert_eq!(response.name, "John Doe");
    assert_eq!(response.age, 30);

    server
        .assert()
        .await?;

    Ok(())
}
