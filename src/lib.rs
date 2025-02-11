#![deny(warnings)]
#![warn(rust_2018_idioms)]

use bytes::Buf;
use http::HeaderValue;
use http_body_util::BodyExt;
use hyper::Request;
use hyper_util::rt::TokioIo;
use serde::Deserialize;
use std::collections::HashMap;
use tokio::net::TcpStream;
use url::Url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct DeboaConfig {
    headers: Option<HashMap<&'static str, &'static str>>,
}

pub struct Deboa {
    base_url: &'static str,
    pub config: Option<DeboaConfig>,
}

impl Deboa {
    pub fn new(base_url: &'static str, config: Option<DeboaConfig>) -> Deboa {
        let mut default_headers: HashMap<&'static str, &'static str> = HashMap::from([
            ("Accept", "application/json"),
            ("Content-Type", "application/json"),
        ]);

        if config.is_some() {
            let mut init_config = config.unwrap();

            let headers = init_config.headers;
            if headers.is_some() {
                for (k, v) in headers.unwrap() {
                    default_headers.insert(k, v);
                }
            }

            init_config.headers = Option::from(default_headers);

            Deboa {
                base_url: base_url,
                config: Option::from(init_config),
            }
        } else {
            Deboa {
                base_url: base_url,
                config: Option::from(DeboaConfig {
                    headers: Option::from(default_headers),
                }),
            }
        }
    }

    pub fn set_base_url(&mut self, base_url: &'static str) {
        self.base_url = base_url
    }

    pub async fn post(
        self,
        path: &str,
        data: Option<HashMap<&str, &str>>,
        config: Option<DeboaConfig>,
    ) -> Result<impl Buf> {
        self.any("POST", path, data, config).await
    }

    pub async fn get(
        self,
        path: &str,
        params: Option<HashMap<&str, &str>>,
        config: Option<DeboaConfig>,
    ) -> Result<impl Buf> {
        self.any("GET", path, params, config).await
    }

    pub async fn put(
        self,
        path: &str,
        data: Option<HashMap<&str, &str>>,
        config: Option<DeboaConfig>,
    ) -> Result<impl Buf> {
        self.any("PUT", path, data, config).await
    }

    pub async fn patch(
        self,
        path: &str,
        data: Option<HashMap<&str, &str>>,
        config: Option<DeboaConfig>,
    ) -> Result<impl Buf> {
        self.any("PATCH", path, data, config).await
    }

    pub async fn delete(self, path: &str) -> Result<impl Buf> {
        self.any("DELETE", path, None, None).await
    }

    pub async fn head(self, path: &str) -> Result<impl Buf> {
        self.any("HEAD", path, None, None).await
    }

    pub async fn any(
        self,
        method: &str,
        path: &str,
        params: Option<HashMap<&str, &str>>,
        config: Option<DeboaConfig>,
    ) -> Result<impl Buf> {
        let mut url = Url::parse(format!("{}{}", self.base_url, path).as_str()).unwrap();

        if method.eq_ignore_ascii_case("GET") {
            if params.is_some() {
                let req_params = params.clone();
                for (key, value) in req_params.unwrap() {
                    url.query_pairs_mut().append_pair(key, value);
                }
            }
        }

        let host = url.host().expect("uri has no host");
        let port = url.port().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method)
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if config.is_some() {
                let req_config = config.unwrap();
                if req_config.headers.is_some() {
                    let headers = req_config.headers.unwrap();
                    for (key, value) in headers {
                        req_headers.insert(key, HeaderValue::from_static(value));
                    }
                }
            }
        }

        let body: String = if method == "GET" {
            "".to_owned()
        } else {
            let req_params = params.clone();
            serde_json::to_string(&req_params).unwrap()
        };

        print!("{body}");

        let req = builder.body(body).unwrap();

        let res = sender.send_request(req).await?;

        let body = res.collect().await?.aggregate();

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);
        let res = api.get("/posts", None, None).await;

        let posts: std::result::Result<Vec<Post>, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("posts: {:#?}", posts);

        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_get_by_query() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let query_map = HashMap::from([("id", "1")]);

        let res = api.get("/comments", Some(query_map), None).await;

        let posts: std::result::Result<Vec<Comment>, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("comments: {:#?}", posts);

        assert_eq!(posts.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_post() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", "1"),
            ("title", "Test"),
            ("body", "Some test to do"),
            ("userId", "1"),
        ]);

        let res = api.post("/posts", Some(body_map), None).await;

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("posts: {:#?}", posts);

        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_put() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", "1"),
            ("title", "Test"),
            ("body", "Some test to do"),
            ("userId", "1"),
        ]);

        let res = api.put("/posts/1", Some(body_map), None).await;

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("posts: {:#?}", posts);

        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_patch() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", "1"),
            ("title", "Test"),
            ("body", "Some test to do"),
            ("userId", "1"),
        ]);

        let res = api.patch("/posts/1", Some(body_map), None).await;

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("posts: {:#?}", posts);

        assert_eq!(1, 1);
    }

    #[tokio::test]
    async fn test_delete() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let res = api.delete("/posts/1").await;

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(res.unwrap().reader());

        println!("posts: {:#?}", posts);

        assert_eq!(1, 1);
    }
}

#[derive(Deserialize, Debug)]
struct Post {
    #[allow(unused)]
    id: i32,
    #[allow(unused)]
    title: String,
    #[allow(unused)]
    body: String,
}

#[derive(Deserialize, Debug)]
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
