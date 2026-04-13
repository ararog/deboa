use crate::{ForyRequestBuilder, ForyResponse};
use deboa::{
    cert::{Certificate, ContentEncoding},
    request::post,
    Client,
};
use deboa_tests::utils::{start_mock_server, Response, CA_CERT};

use fory::{Fory, ForyObject};
use http::StatusCode;

const FORY_PERSON: [u8; 15] = [2, 255, 143, 2, 30, 255, 34, 74, 111, 104, 110, 32, 68, 111, 101];

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

async fn do_fory_post_request() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/posts";

    let mut server = start_mock_server(|req| async move {
        if req.method() == "POST" && req.uri().path() == "/posts" {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .bytes(&FORY_PERSON))
        } else {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .bytes(&[]))
        }
    })
    .await;

    let client = Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .build();

    let mut fory = Fory::default();
    let result = fory.register::<Person>(1);
    assert!(result.is_ok());

    let person = Person { name: "John Doe".to_string(), age: 30 };

    let request = post(server.url(path))?.body_as_fory(&fory, person)?;

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
