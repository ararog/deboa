use crate::Vamo;
use deboa::HttpVersion;
use http::Method;

#[test]
fn test_create_vamo() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");

    assert_eq!(vamo.base_url, "https://jsonplaceholder.typicode.com");
}

#[test]
fn test_client() {
    let mut vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    assert_eq!(vamo.client().protocol(), &HttpVersion::Http1);
}

#[test]
fn test_get() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    let response = vamo.get("/posts").build().unwrap();
    assert_eq!(response.method(), Method::GET);
}

#[test]
fn test_post() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    let response = vamo.post("/posts").build().unwrap();
    assert_eq!(response.method(), Method::POST);
}

#[test]
fn test_put() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    let response = vamo.put("/posts").build().unwrap();
    assert_eq!(response.method(), Method::PUT);
}

#[test]
fn test_patch() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    let response = vamo.patch("/posts").build().unwrap();
    assert_eq!(response.method(), Method::PATCH);
}

#[test]
fn test_delete() {
    let vamo = Vamo::new("https://jsonplaceholder.typicode.com");
    let response = vamo.delete("/posts").build().unwrap();
    assert_eq!(response.method(), Method::DELETE);
}
