use crate::http::json::{JsonRequest, JsonResponse};
use deboa::Deboa;
use deboa::errors::DeboaError;

use http::header;

use crate::tests::types::{JSON_POST, JSONPLACEHOLDER, Post, sample_post};

use httpmock::MockServer;

#[cfg(feature = "json")]
#[test]
fn test_set_json() -> Result<(), DeboaError> {
    let mut api = Deboa::new(JSONPLACEHOLDER)?;

    let data = sample_post();

    let _ = api.set_json(data);

    assert_eq!(*api.body(), JSON_POST.to_vec());

    Ok(())
}

#[cfg(feature = "json")]
#[tokio::test]
async fn test_response_json() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = sample_post();

    let http_mock = server.mock(|when, then| {
        use http::StatusCode;

        when.method(http::Method::GET.as_str()).path("/posts/1");
        then.status(StatusCode::OK.into())
            .header(header::CONTENT_TYPE.as_str(), mime::APPLICATION_JSON.to_string())
            .body(JSON_POST);
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let api = Deboa::new(&format!("http://{ip}:{port}"));

    let response = api?.get("posts/1").await?.json::<Post>()?;

    http_mock.assert();

    assert_eq!(response, data);

    Ok(())
}
