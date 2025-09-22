use deboa::errors::DeboaError;
use http::StatusCode;
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};

use crate::Vamo;

#[tokio::test]
async fn test_get() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/posts");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.get("/posts")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PUT).path("/posts");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.put("/posts")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(POST).path("/posts");
        then.status(StatusCode::CREATED.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.post("/posts")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(PATCH).path("/posts/1");
        then.status(StatusCode::OK.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.patch("/posts/1")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(DELETE).path("/posts/1");
        then.status(StatusCode::NO_CONTENT.into());
    });

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.delete("/posts/1")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}
