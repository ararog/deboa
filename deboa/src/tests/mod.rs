#[cfg(test)]
pub mod deboa_tests {
    use crate::DeboaError;
    #[cfg(feature = "middlewares")]
    use crate::{middlewares::DeboaMiddleware, response::DeboaResponse, Deboa};

    use http::{header, StatusCode};
    #[cfg(feature = "json")]
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
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

    #[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct Response {
        response_code: i32,
        response_message: String,
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

        api.add_header(header::CONTENT_TYPE, "application/json".to_string());
        
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
        let mut api: Deboa = Deboa::new("https://jsonplaceholder.typicode.com");

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

    #[test]
    fn test_base_url() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        assert_eq!(api.base_url, "https://jsonplaceholder.typicode.com");
    }

    #[test]
    fn test_set_query_params() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let query_map = HashMap::from([("id", "1")]);

        api.set_query_params(Some(query_map.clone()));

        assert_eq!(api.query_params, Some(query_map));
    }

    #[test]
    fn test_set_headers() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let headers = HashMap::from([(header::CONTENT_TYPE, "application/json".to_string())]);

        api.headers = Some(headers);

        assert_eq!(api.headers, Some(HashMap::from([(header::CONTENT_TYPE, "application/json".to_string())])));
    }

    #[test]
    fn test_set_basic_auth() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.add_basic_auth("username".to_string(), "password".to_string());

        assert_eq!(
            api.get_mut_header(&header::AUTHORIZATION),
            Some(&mut "Basic dXNlcm5hbWU6cGFzc3dvcmQ=".to_string())
        );
    }

    #[test]
    fn test_set_bearer_auth() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.add_bearer_auth("token".to_string());

        assert_eq!(api.get_mut_header(&header::AUTHORIZATION), Some(&mut "Bearer token".to_string()));
    }

    #[test]
    fn test_set_retries() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.set_retries(5);

        assert_eq!(api.retries, 5);
    }

    #[test]
    fn test_set_connection_timeout() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.set_connection_timeout(5);

        assert_eq!(api.connection_timeout, 5);
    }

    #[test]
    fn test_set_request_timeout() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.set_request_timeout(5);

        assert_eq!(api.request_timeout, 5);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_set_json() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

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
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        let data = Post {
            id: 1,
            title: "sunt aut facere repellat provident occaecati excepturi optio reprehenderit".to_string(),
            body: "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto".to_string(),
        };

        let response = api.get("/posts/1").await?.json::<Post>().await?;

        assert_eq!(response, data);

        Ok(())
    }

    #[cfg(all(feature = "xml", feature = "tokio-rt"))]
    #[tokio::test]
    async fn test_set_xml() -> Result<(), DeboaError> {
        let mut api = Deboa::new("https://reqbin.com");

        let data = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        let _ = api.set_xml(data)?.get("/echo/get/xml").await?;

        assert_eq!(
            api.body,
            Some("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Post><id>1</id><title>Test</title><body>Some test to do</body></Post>".to_string())
        );

        Ok(())
    }

    #[cfg(all(feature = "xml", feature = "tokio-rt"))]
    #[tokio::test]
    /*
    async fn test_xml_response() -> Result<(), DeboaError> {
        let mut api = Deboa::new("https://reqbin.com");
        api.edit_header(header::CONTENT_TYPE, crate::APPLICATION_XML.to_string());
        api.edit_header(header::ACCEPT, crate::APPLICATION_XML.to_string());

        let response = api.get("/echo/get/xml").await?.xml::<Response>().await?;

        assert_eq!(response.response_code, 200);
        Ok(())
    }
    */
    #[cfg(feature = "msgpack")]
    #[test]
    fn test_set_msgpack() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

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
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.edit_header(header::CONTENT_TYPE, "application/json".to_string());

        assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
    }

    #[test]
    fn test_add_header() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.add_header(header::CONTENT_TYPE, "application/json".to_string());

        assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
    }

    #[test]
    fn test_remove_header() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.remove_header(header::CONTENT_TYPE);

        assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), None);
    }

    #[test]
    fn test_set_body() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.set_text("test".to_string());

        assert_eq!(api.body, Some(b"test".to_vec()));
    }

    #[test]
    fn test_get_mut_header() {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        api.add_header(header::CONTENT_TYPE, "application/json".to_string());

        assert_eq!(api.get_mut_header(&header::CONTENT_TYPE), Some(&mut "application/json".to_string()));
    }
}
