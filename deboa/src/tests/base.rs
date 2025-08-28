#[cfg(feature = "middlewares")]
use crate::Deboa;
use crate::DeboaError;

use crate::tests::types::Post;
use http::header;
use std::collections::HashMap;

use httpmock::prelude::*;

#[cfg(feature = "smol-rt")]
use macro_rules_attribute::apply;
#[cfg(feature = "smol-rt")]
use smol_macros::test;

#[test]
fn test_base_url() {
    let api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    assert_eq!(api.base_url, "https://jsonplaceholder.typicode.com");
}

#[test]
fn test_set_query_params() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    let query_map = HashMap::from([("id", "1")]);

    api.set_query_params(Some(query_map.clone()));

    assert_eq!(api.query_params, Some(query_map));
}

#[test]
fn test_set_headers() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    let headers = HashMap::from([(header::CONTENT_TYPE, "application/json".to_string())]);

    api.headers = Some(headers);

    assert_eq!(api.headers, Some(HashMap::from([(header::CONTENT_TYPE, "application/json".to_string())])));
}

#[test]
fn test_set_basic_auth() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.add_basic_auth("username".to_string(), "password".to_string());

    assert_eq!(
        api.get_mut_header(&header::AUTHORIZATION),
        Some(&mut "Basic dXNlcm5hbWU6cGFzc3dvcmQ=".to_string())
    );
}

#[test]
fn test_set_bearer_auth() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.add_bearer_auth("token".to_string());

    assert_eq!(api.get_mut_header(&header::AUTHORIZATION), Some(&mut "Bearer token".to_string()));
}

#[test]
fn test_set_retries() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.set_retries(5);

    assert_eq!(api.retries, 5);
}

#[test]
fn test_set_connection_timeout() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.set_connection_timeout(5);

    assert_eq!(api.connection_timeout, 5);
}

#[test]
fn test_set_request_timeout() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.set_request_timeout(5);

    assert_eq!(api.request_timeout, 5);
}

#[cfg(feature = "json")]
#[test]
fn test_set_json() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    let data = Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    };

    let _ = api.set_json(data);

    assert_eq!(api.body, Some(b"{\"id\":1,\"title\":\"Test\",\"body\":\"Some test to do\"}".to_vec()));
}

#[cfg(feature = "json")]
#[tokio::test]
async fn test_response_json() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = Post {
        id: 1,
        title: "sunt aut".to_string(),
        body: "quia et".to_string(),
    };

    let http_mock = server.mock(|when, then| {
        use serde_json::json;

        when.method(GET).path("/posts/1");
        then.status(200).header("content-type", "application/json").body(
            json!({
                "id": 1,
                "title": "sunt aut",
                "body": "quia et"
            })
            .to_string(),
        );
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let api = Deboa::new(format!("http://{ip}:{port}").to_string());

    let response = api.get("/posts/1").await?.json::<Post>().await?;

    http_mock.assert();

    assert_eq!(response, data);

    Ok(())
}

#[cfg(all(feature = "xml", feature = "tokio-rt"))]
#[tokio::test]
async fn test_set_xml() -> Result<(), DeboaError> {
    let mut api = Deboa::new("https://reqbin.com".to_string());

    let data = Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    };

    let _ = api.set_xml(data)?.get("/echo/get/xml").await?;

    assert_eq!(
        api.body,
        Some(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><Post><id>1</id><title>Test</title><body>Some test to do</body></Post>".to_vec())
    );

    Ok(())
}

#[cfg(all(feature = "xml", feature = "tokio-rt"))]
#[tokio::test]
async fn test_xml_response() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let data = Post {
        id: 1,
        title: "sunt aut".to_string(),
        body: "quia et".to_string(),
    };

    let http_mock = server.mock(|when, then| {
        when.method(GET).path("/posts/1");
        then.status(200)
            .header("content-type", "application/xml")
            .body(b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><Post><id>1</id><title>sunt aut</title><body>quia et</body></Post>");
    });

    let server_address = *server.address();

    let ip = server_address.ip();
    let port = server_address.port();

    let mut api = Deboa::new(format!("http://{ip}:{port}").to_string());
    api.edit_header(header::CONTENT_TYPE, crate::APPLICATION_XML.to_string());
    api.edit_header(header::ACCEPT, crate::APPLICATION_XML.to_string());

    let response = api.get("/posts/1").await?.xml::<Post>().await?;

    http_mock.assert();

    assert_eq!(response, data);
    Ok(())
}

#[cfg(feature = "msgpack")]
#[test]
fn test_set_msgpack() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    let data = Post {
        id: 1,
        title: "Test".to_string(),
        body: "Some test to do".to_string(),
    };

    let _ = api.set_msgpack(data);

    assert_eq!(api.body, Some("{\"id\":1,\"title\":\"Test\",\"body\":\"Some test to do\"}".to_string()));
}

#[test]
fn test_edit_header() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.edit_header(header::CONTENT_TYPE, "application/json".to_string());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
}

#[test]
fn test_add_header() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.add_header(header::CONTENT_TYPE, "application/json".to_string());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
}

#[test]
fn test_remove_header() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.remove_header(header::CONTENT_TYPE);

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), None);
}

#[test]
fn test_set_body() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.set_text("test".to_string());

    assert_eq!(api.body, Some(b"test".to_vec()));
}

#[test]
fn test_get_mut_header() {
    let mut api = Deboa::new("https://jsonplaceholder.typicode.com".to_string());

    api.add_header(header::CONTENT_TYPE, "application/json".to_string());

    assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
}
