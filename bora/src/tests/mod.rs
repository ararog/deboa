use crate::Bora;
use deboa::HttpVersion;
use http::Method;

#[test]
fn test_create_bora() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");

    assert_eq!(bora.base_url, "https://jsonplaceholder.typicode.com");
}

#[test]
fn test_client() {
    let mut bora = Bora::new("https://jsonplaceholder.typicode.com");
    assert_eq!(bora.client().protocol(), &HttpVersion::Http1);
}

#[test]
fn test_get() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");
    let response = bora.get("/posts").build().unwrap();
    assert_eq!(response.method(), Method::GET);
}

#[test]
fn test_post() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");
    let response = bora.post("/posts").build().unwrap();
    assert_eq!(response.method(), Method::POST);
}

#[test]
fn test_put() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");
    let response = bora.put("/posts").build().unwrap();
    assert_eq!(response.method(), Method::PUT);
}

#[test]
fn test_patch() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");
    let response = bora.patch("/posts").build().unwrap();
    assert_eq!(response.method(), Method::PATCH);
}

#[test]
fn test_delete() {
    let bora = Bora::new("https://jsonplaceholder.typicode.com");
    let response = bora.delete("/posts").build().unwrap();
    assert_eq!(response.method(), Method::DELETE);
}
