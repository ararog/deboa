use crate::Client;
use deboa::{
    errors::{DeboaError, DnsError, ResponseError},
    request::{DeboaRequest, FetchWith, IntoRequest},
    response::DeboaResponse,
    HttpClient,
};
use http::StatusCode;

//
// GET
//

#[compio::test]
async fn test_get_http() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let request = DeboaRequest::get("https://jsonplaceholder.typicode.com/posts/1")?.build()?;

    let response: DeboaResponse = client
        .execute(request)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    Ok(())
}

//
// GET NOT FOUND
//

#[compio::test]
async fn test_get_not_found() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let response: crate::Result<DeboaResponse> =
        DeboaRequest::get("https://jsonplaceholder.typicode.com/asasa/posts/1ddd")?
            .send_with(&client)
            .await;

    assert!(response.is_err());
    assert_eq!(
        response.unwrap_err(),
        DeboaError::Response(ResponseError::Receive {
            status_code: StatusCode::NOT_FOUND,
            message: "Could not process request (404 Not Found): Not found".to_string()
        })
    );

    Ok(())
}

//
// GET INVALID SERVER
//

#[compio::test]
async fn test_get_invalid_server() -> Result<(), Box<dyn std::error::Error>> {
    let api = Client::default();

    let request = DeboaRequest::get("https://invalid-server.com/posts")?
        .text("test")
        .build()?;

    let response: crate::Result<DeboaResponse> = api
        .execute(request)
        .await;

    let error = DeboaError::Dns(DnsError::Resolve {
        host: "invalid-server.com".to_string(),
        message: "failed to lookup address information: Name or service not known".to_string(),
    });

    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), error);

    Ok(())
}

//
// GET BY QUERY
//

#[compio::test]
async fn test_get_by_query() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();

    let response = DeboaRequest::get("https://jsonplaceholder.typicode.com/comments/1")?
        .send_with(&client)
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "Status code is {} and should be {}",
        response
            .status()
            .as_u16(),
        StatusCode::OK.as_u16()
    );

    let comments = response
        .text()
        .await;

    assert!(comments.is_ok());
    assert_eq!(comments.unwrap(), "{\n  \"postId\": 1,\n  \"id\": 1,\n  \"name\": \"id labore ex et quam laborum\",\n  \"email\": \"Eliseo@gardner.biz\",\n  \"body\": \"laudantium enim quasi est quidem magnam voluptate ipsam eos\\ntempora quo necessitatibus\\ndolor quam autem quasi\\nreiciendis et nam sapiente accusantium\"\n}");

    Ok(())
}

/*
async fn do_get_by_query_with_retries() -> Result<()> {
    let mut server = start_mock_server(|_req| async move {
        Ok(make_response(StatusCode::BAD_GATEWAY, "pong"))
    })
    .await;

    let client = client_with_cert();

    let response = DeboaRequest::get(server.url("/comments/1"))?
        .retries(2)
        .send_with(client)
        .await;

    if let Err(err) = response {
        assert_eq!(
            err,
            DeboaError::Response(ResponseError::Receive {
                status_code: StatusCode::BAD_GATEWAY,
                message: "Could not process request (502 Bad Gateway): pong".to_string(),
            }),
        );
    }

    server.stop().await;

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_by_query_with_retries() -> Result<(), Box<dyn std::error::Error>> {
    do_get_by_query_with_retries().await
}

#[cfg(feature = "smol-rt")]
#[compio::test]
async fn test_get_by_query_with_retries() {
    let _ = do_get_by_query_with_retries().await;
}
*/

/*
async fn do_get_with_redirect() -> Result<()> {
    let client = Client::default();

    let url = if cfg!(feature = "http3-tokio") {
        "https://tinyurl.com/bccjpjd7"
    } else {
        "https://tinyurl.com/bp6e548"
    };

    let response = DeboaRequest::get(url)?
        .send_with(client)
        .await?;

    let server = if cfg!(feature = "http3-tokio") { "facebook.com" } else { "github.com" };

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("server")
            .unwrap()
            .to_str()
            .unwrap(),
        server
    );

    Ok(())
}

#[cfg(feature = "tokio-rt")]
#[tokio::test]
async fn test_get_with_redirect() -> Result<(), Box<dyn std::error::Error>> {
    do_get_with_redirect().await
}

#[cfg(feature = "smol-rt")]
#[compio::test]
async fn test_get_with_redirect() {
    let _ = do_get_with_redirect().await;
}
*/

#[compio::test]
async fn test_try_into() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let first_post = "https://jsonplaceholder.typicode.com/posts/1";
    let response = client
        .execute(first_post.into_request()?)
        .await?;
    assert_eq!(response.status(), 200);

    Ok(())
}

#[compio::test]
async fn test_fetch_from_str() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::default();
    let first_post = "https://jsonplaceholder.typicode.com/posts/1";
    let response = first_post
        .fetch_with(client)
        .await?;
    assert_eq!(response.status(), 200);
    Ok(())
}
