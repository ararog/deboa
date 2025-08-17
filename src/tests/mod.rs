#[cfg(test)]
pub mod deboa_tests {
    use crate::Deboa;
    use crate::StatusCode;

    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Default, Serialize, Deserialize, Debug)]
    struct Post {
        #[allow(unused)]
        id: i32,
        #[allow(unused)]
        title: String,
        #[allow(unused)]
        body: String,
    }

    #[derive(Default, Serialize, Deserialize, Debug)]
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

    //
    // GET
    //

    async fn do_get() -> Result<()> {
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
    async fn test_get() -> Result<()> {
        do_get().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get() {
        let _ = do_get().await;
    }

    //
    // GET BY QUERY
    //

    async fn do_get_by_query() -> Result<()> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let query_map = HashMap::from([("id", "1")]);

        api.set_query(Some(query_map));

        let mut response = api.get("/comments").await?;

        assert_eq!(
            response.status,
            StatusCode::OK,
            "Status code is {} and should be {}",
            response.status.as_u16(),
            StatusCode::OK.as_u16()
        );

        let comments = response.json::<Vec<Comment>>().await?;

        assert_eq!(
            comments.len(),
            1,
            "Number of comments is {} and should be {}",
            comments.len(),
            1
        );

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_get_by_query() -> Result<()> {
        do_get_by_query().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_get_by_query() {
        let _ = do_get_by_query().await;
    }

    //
    // POST
    //

    async fn do_post() -> Result<()> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let data = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        let response = api.set_json(data).post("/posts").await?;

        assert_eq!(response.status, StatusCode::CREATED);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_post() -> Result<()> {
        do_post().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_post() {
        let _ = do_post().await;
    }

    //
    // PUT
    //

    async fn do_put() -> Result<()> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let post = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        let response = api.set_json(post).put("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_put() -> Result<()> {
        do_put().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_put() {
        let _ = do_put().await;
    }

    //
    // PATCH
    //

    async fn do_patch() -> Result<()> {
        let mut api = Deboa::new("https://jsonplaceholder.typicode.com");

        let data = Post {
            id: 1,
            title: "Test".to_string(),
            body: "Some test to do".to_string(),
        };

        let response = api.set_json(data).patch("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_patch() -> Result<()> {
        do_patch().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_patch() {
        let _ = do_patch().await;
    }

    //
    // DELETE
    //

    async fn do_delete() -> Result<()> {
        let api = Deboa::new("https://jsonplaceholder.typicode.com");

        let response = api.delete("/posts/1").await?;

        assert_eq!(response.status, StatusCode::OK);

        Ok(())
    }

    #[cfg(feature = "tokio-rt")]
    #[tokio::test]
    async fn test_delete() -> Result<()> {
        do_delete().await?;
        Ok(())
    }

    #[cfg(feature = "smol-rt")]
    #[apply(test!)]
    async fn test_delete() {
        let _ = do_delete().await;
    }
}
