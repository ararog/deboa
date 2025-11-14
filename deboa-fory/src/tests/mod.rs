use crate::{ForyRequestBuilder, ForyResponse};
use deboa::{errors::DeboaError, request::post, Deboa};
use fory::{Fory, ForyObject};
use http::header;
use httpmock::{Method, MockServer};

const FORY_PERSON: [u8; 15] = [2, 255, 143, 2, 30, 255, 34, 74, 111, 104, 110, 32, 68, 111, 101];

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::test]
async fn test_fory_post_request() -> Result<(), DeboaError> {
    let client = Deboa::new();

    let server = MockServer::start();
    let method = Method::POST;
    let path = "/post";
    let status = 200;

    let mock = server.mock(|when, then| {
        when.method(method)
            .path(path);
        then.status::<u16>(status)
            .header(header::CONTENT_TYPE.as_str(), "application/fory")
            .body(FORY_PERSON);
    });

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

    mock.assert();

    assert_eq!(response.name, "John Doe");
    assert_eq!(response.age, 30);

    Ok(())
}
