#[cfg(test)]
pub mod deboa_tests {
    use crate::DeboaError;
    #[cfg(feature = "middlewares")]
    use crate::{middlewares::DeboaMiddleware, response::DeboaResponse, Deboa};
    use http::StatusCode;
    #[cfg(feature = "json")]
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Default, Serialize, Deserialize, Debug)]
    #[cfg(feature = "json")]
    struct Post {
        #[allow(unused)]
        id: i32,
        #[allow(unused)]
        title: String,
        #[allow(unused)]
        body: String,
    }

    #[derive(Default, Serialize, Deserialize, Debug)]
    #[cfg(feature = "json")]
    struct Comment {
        #[allow(unused)]
        id: i32,
        #[allow(unused)]
        name: String,
        #[allow(unused)]
        email: String,
        #[allow(unused)]
        body: String,
    }

    #[cfg(feature = "smol-rt")]
    use macro_rules_attribute::apply;
    #[cfg(feature = "smol-rt")]
    use smol_macros::test;

    #[cfg(feature = "middlewares")]
    #[derive(Default)]
    struct TestMonitor;

    #[cfg(feature = "middlewares")]
    impl DeboaMiddleware for TestMonitor {
        fn on_request(&self, request: &Deboa) {
            println!("Request: {request:?}");
        }

        fn on_response(&self, _request: &Deboa, response: &mut DeboaResponse) {
            println!("Response: {response:?}");
        }
    }

    //
    // MIDDLEWARES
    //

    #[cfg(feature = "middlewares")]
    async fn do_middleware() -> Result<(), DeboaError> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.add_middleware(TestMonitor);

        let response = api.get("/posts/1").await?;

        assert_eq!(
            response.status,
            StatusCode::OK,
            "Status code is {} and should be {}",
            response.status.as_u16(),
            StatusCode::OK.as_u16()
        );

        Ok(())
    }

    #[cfg(all(feature = "tokio-rt", feature = "middlewares"))]
    #[tokio::test]
    async fn test_middleware() -> Result<(), DeboaError> {
        do_middleware().await?;
        Ok(())
    }

    #[cfg(all(feature = "smol-rt", feature = "middlewares"))]
    #[apply(test!)]
    async fn test_middleware() {
        let _ = do_middleware().await;
    }

    #[cfg(all(feature = "compio-rt", feature = "middlewares"))]
    #[compio::test]
    async fn test_middleware() {
        let _ = do_middleware().await;
    }

    //
    // GET
    //

    async fn do_get() -> Result<(), DeboaError> {
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        let response = api.get("/posts").await?;

        assert_eq!(
            response.status,
            StatusCode::OK,
            "Status code is {} and should be {}",
            response.status.as_u16(),
            StatusCode::OK.as_u16()
        );

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_get() -> Result<(), DeboaError> {
        do_get().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get() {
        let _ = do_get().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_get() {
        let _ = do_get().await;
    }

    //
    // GET NOT FOUND
    //

    async fn do_get_not_found() -> Result<(), DeboaError> {
        let api = Deboa::new("https://jsonplaceholder.typicode.com/dsdsd");

        let response = api.get("/posts").await?;

        assert_eq!(
            response.status,
            StatusCode::NOT_FOUND,
            "Status code is {} and should be {}",
            response.status.as_u16(),
            StatusCode::NOT_FOUND.as_u16()
        );

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_get_not_found() -> Result<(), DeboaError> {
        do_get_not_found().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get_not_found() {
        let _ = do_get_not_found().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_get_not_found() {
        let _ = do_get_not_found().await;
    }

    //
    // GET INVALID SERVER
    //

    async fn do_get_invalid_server() -> Result<(), DeboaError> {
        let api = Deboa::new("https://invalid-server.com");

        let response = api.get("/posts").await;

        assert!(response.is_err());
        assert_eq!(
            response,
            Err(DeboaError::ConnectionError {
                host: "invalid-server.com".to_string(),
                message: "failed to lookup address information: Name or service not known".to_string(),
            })
        );

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_get_invalid_server() -> Result<(), DeboaError> {
        do_get_invalid_server().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get_invalid_server() {
        let _ = do_get_invalid_server().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_get_invalid_server() {
        let _ = do_get_invalid_server().await;
    }

    //
    // GET BY QUERY
    //

    async fn do_get_by_query() -> Result<(), DeboaError> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let query_map = HashMap::from([("id", "1")]);

        api.set_query_params(Some(query_map));

        #[cfg(feature = "json")]
        let mut response = api.get("/comments").await?;

        #[cfg(not(feature = "json"))]
        let response = api.get("/comments").await?;

        assert_eq!(
            response.status,
            StatusCode::OK,
            "Status code is {} and should be {}",
            response.status.as_u16(),
            StatusCode::OK.as_u16()
        );

        #[cfg(feature = "json")]
        let comments = response.json::<Vec<Comment>>().await?;

        #[cfg(feature = "json")]
        assert_eq!(comments.len(), 1, "Number of comments is {} and should be {}", comments.len(), 1);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_get_by_query() -> Result<(), DeboaError> {
        do_get_by_query().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get_by_query() {
        let _ = do_get_by_query().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_get_by_query() {
        let _ = do_get_by_query().await;
    }

    //
    // POST
    //

    async fn do_post() -> Result<(), DeboaError> {
        #[cfg(feature = "json")]
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(not(feature = "json"))]
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(feature = "json")]
        let data = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        #[cfg(feature = "json")]
        let response = api.set_json(data)?.post("/posts").await?;

        #[cfg(not(feature = "json"))]
        let response = api.post("/posts").await?;

        assert_eq!(response.status, StatusCode::CREATED);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_post() -> Result<(), DeboaError> {
        do_post().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_post() {
        let _ = do_post().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_post() {
        let _ = do_post().await;
    }

    //
    // PUT
    //

    async fn do_put() -> Result<(), DeboaError> {
        #[cfg(feature = "json")]
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(not(feature = "json"))]
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(feature = "json")]
        let post = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        #[cfg(feature = "json")]
        let response = api.set_json(post)?.put("/posts/1").await?;

        #[cfg(not(feature = "json"))]
        let response = api.put("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_put() -> Result<(), DeboaError> {
        do_put().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_put() {
        let _ = do_put().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_put() {
        let _ = do_put().await;
    }

    //
    // PATCH
    //

    async fn do_patch() -> Result<(), DeboaError> {
        #[cfg(feature = "json")]
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(not(feature = "json"))]
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        #[cfg(feature = "json")]
        let data = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        #[cfg(feature = "json")]
        let response = api.set_json(data)?.patch("/posts/1").await?;

        #[cfg(not(feature = "json"))]
        let response = api.patch("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_patch() -> Result<(), DeboaError> {
        do_patch().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_patch() {
        let _ = do_patch().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_patch() {
        let _ = do_patch().await;
    }

    //
    // DELETE
    //

    async fn do_delete() -> Result<(), DeboaError> {
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        let response = api.delete("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_delete() -> Result<(), DeboaError> {
        do_delete().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_delete() {
        let _ = do_delete().await;
    }

    #[cfg(feature = "compio-rt")]
    #[compio::test]
    async fn test_delete() {
        let _ = do_delete().await;
    }
}
