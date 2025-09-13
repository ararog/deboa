use http::StatusCode;
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};

use crate::Vamo;

#[tokio::test]
async fn test_get() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/posts");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str());
    let response = vamo.get("/posts").send_with(vamo.client()).await.unwrap();

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_put() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT).path("/posts");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str());
    let response = vamo.put("/posts").send_with(vamo.client()).await.unwrap();

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_post() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/posts");
        then.status(StatusCode::CREATED.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str());
    let response = vamo.post("/posts").send_with(vamo.client()).await.unwrap();

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_patch() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PATCH).path("/posts/1");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str());
    let response = vamo.patch("/posts/1").send_with(vamo.client()).await.unwrap();

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_delete() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/posts/1");
        then.status(StatusCode::NO_CONTENT.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str());
    let response = vamo.delete("/posts/1").send_with(vamo.client()).await.unwrap();

    mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
