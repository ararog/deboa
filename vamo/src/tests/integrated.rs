use deboa::errors::DeboaError;
use deboa_tests::utils::setup_server;
use http::StatusCode;
use httpmock::{
    Method::{DELETE, GET, PATCH, POST, PUT},
    MockServer,
};

use crate::Vamo;

#[tokio::test]
async fn test_get() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", GET, StatusCode::OK);

    let mut vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.get("/posts")?.go(vamo.client()).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_put() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", PUT, StatusCode::OK);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.put("/posts")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_post() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts", POST, StatusCode::CREATED);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.post("/posts")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::CREATED);

    Ok(())
}

#[tokio::test]
async fn test_patch() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts/1", PATCH, StatusCode::OK);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.patch("/posts/1")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::OK);

    Ok(())
}

#[tokio::test]
async fn test_delete() -> Result<(), DeboaError> {
    let server = MockServer::start();
    let mock = setup_server(&server, "/posts/1", DELETE, StatusCode::NO_CONTENT);

    let vamo = Vamo::new(server.base_url().as_str())?;
    let response = vamo.delete("/posts/1")?.go(vamo).await?;

    mock.assert();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    Ok(())
}
