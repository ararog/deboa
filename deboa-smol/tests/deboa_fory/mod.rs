use std::error::Error;

use deboa::request::post;
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use easyhttpmock_vetis_smol::mock::{MethodExt, Mock, StatusCodeExt};
use fory::{Fory, ForyObject};
use http::StatusCode;
use macro_rules_attribute::apply;
use smol_macros::test;

use crate::common::helpers::{client_with_cert, start_mock_server};

const FORY_PERSON: [u8; 15] = [2, 255, 143, 2, 30, 255, 34, 74, 111, 104, 110, 32, 68, 111, 101];

#[derive(ForyObject)]
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

    let client = client_with_cert();

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
