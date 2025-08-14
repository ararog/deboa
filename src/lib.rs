#![deny(warnings)]
#![warn(rust_2018_idioms)]

use anyhow::{bail, Result};
use async_native_tls::TlsStream;
use bytes::Buf;
use http::{HeaderMap, HeaderValue, StatusCode};
use http_body_util::BodyExt;
use hyper::Request;
use serde::{Deserialize, Serialize};
use smol::{io, net::TcpStream, prelude::*};
use smol_hyper::rt::FuturesIo;
use std::task::{Context, Poll};
use std::{collections::HashMap, fmt::Display, pin::Pin};
use url::Url;

pub struct DeboaConfig {
    headers: Option<HashMap<&'static str, &'static str>>,
}

pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
}

pub struct Deboa {
    base_url: &'static str,
    pub config: Option<DeboaConfig>,
}

impl Deboa {
    pub fn new(base_url: &'static str, config: Option<DeboaConfig>) -> Self {
        let mut default_headers: HashMap<&'static str, &'static str> = HashMap::from([
            ("Accept", "application/json"),
            ("Content-Type", "application/json"),
        ]);

        match config {
            Some(mut config) => {
                if let Some(headers) = config.headers {
                    headers
                        .into_iter()
                        .fold(&mut default_headers, |acc, (key, value)| {
                            acc.insert(key, value);
                            acc
                        });
                };

                config.headers = Option::from(default_headers);

                Deboa {
                    base_url,
                    config: Option::from(config),
                }
            }
            None => Deboa {
                base_url,
                config: Option::from(DeboaConfig {
                    headers: Option::from(default_headers),
                }),
            },
        }
    }

    pub fn set_base_url(&mut self, base_url: &'static str) {
        self.base_url = base_url
    }

    pub async fn post(
        &self,
        path: &str,
        data: Option<HashMap<&str, RequestValue>>,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::POST, path, data, config).await
    }

    pub async fn get(
        &self,
        path: &str,
        params: Option<HashMap<&str, RequestValue>>,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::GET, path, params, config).await
    }

    pub async fn put(
        &self,
        path: &str,
        data: Option<HashMap<&str, RequestValue>>,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::PUT, path, data, config).await
    }

    pub async fn patch(
        &self,
        path: &str,
        data: Option<HashMap<&str, RequestValue>>,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::PATCH, path, data, config).await
    }

    pub async fn delete(
        &self,
        path: &str,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::DELETE, path, None, config).await
    }

    pub async fn head(
        &self,
        path: &str,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::HEAD, path, None, config).await
    }

    pub async fn options(
        &self,
        path: &str,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::OPTIONS, path, None, config).await
    }

    pub async fn trace(
        &self,
        path: &str,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::TRACE, path, None, config).await
    }

    pub async fn connect(
        &self,
        path: &str,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        self.any(RequestMethod::CONNECT, path, None, config).await
    }

    pub async fn any(
        &self,
        method: RequestMethod,
        path: &str,
        params: Option<HashMap<&str, RequestValue>>,
        config: Option<DeboaConfig>,
    ) -> Result<(DeboaResponse, impl Buf)> {
        let mut url = Url::parse(format!("{}{}", self.base_url, path).as_str()).unwrap();

        let body = match params {
            Some(params) => {
                if method.to_string().eq_ignore_ascii_case("GET") {
                    for (key, value) in params.iter() {
                        url.query_pairs_mut()
                            .append_pair(key, value.to_string().as_str());
                    }
                    "".to_owned()
                } else {
                    serde_json::to_string(&params).unwrap()
                }
            }
            None => "".to_owned(),
        };

        let host = url.host().expect("uri has no host");

        let io = {

            match url.scheme() {
                "http" => {
                    let stream = {
                        let port = url.port().unwrap_or(80);
                        TcpStream::connect((host.to_string(), port)).await?
                    };
                    SmolStream::Plain(stream)
                }
                "https" => {
                    // In case of HTTPS, establish a secure TLS connection first.
                    let stream = {
                        let port = url.port().unwrap_or(443);
                        TcpStream::connect((host.to_string(), port)).await?
                    };
                    let stream = async_native_tls::connect(host.to_string(), stream).await?;
                    SmolStream::Tls(stream)
                }
                scheme => bail!("unsupported scheme: {:?}", scheme),
            }
        };

        let (mut sender, conn) = hyper::client::conn::http1::handshake(FuturesIo::new(io)).await?;
        smol::spawn(async move {
            if let Err(e) = conn.await {
                println!("Connection failed: {e:?}");
            }
        })
        .detach();

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            match config {
                Some(config) => {
                    if let Some(headers) = config.headers {
                        let req_headers = builder.headers_mut().unwrap();
                        for (key, value) in headers.into_iter() {
                            req_headers.insert(key, HeaderValue::from_static(value));
                        }
                    }
                }
                None => {
                    if let Some(config) = &self.config {
                        if let Some(headers) = config.headers.clone() {
                            let req_headers = builder.headers_mut().unwrap();
                            for (key, value) in headers.into_iter() {
                                req_headers.insert(key, HeaderValue::from_static(value));
                            }
                        }
                    }
                }
            }
        }

        let req = builder.body(body).unwrap();

        let res = sender.send_request(req).await?;

        let response = DeboaResponse {
            status: res.status(),
            headers: res.headers().clone(),
        };

        let body = res.collect().await?.aggregate();

        Ok((response, body))
    }
}

enum SmolStream {
    /// A plain TCP connection.
    Plain(TcpStream),

    /// A TCP connection secured by TLS.
    Tls(TlsStream<TcpStream>),
}

impl AsyncRead for SmolStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_read(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for SmolStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_write(cx, buf),
            SmolStream::Tls(stream) => Pin::new(stream).poll_write(cx, buf),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_close(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_close(cx),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match &mut *self {
            SmolStream::Plain(stream) => Pin::new(stream).poll_flush(cx),
            SmolStream::Tls(stream) => Pin::new(stream).poll_flush(cx),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);
        let api_call_result = api.get("/posts", None, None).await;

        if let Err(err) = api_call_result {
            panic!("error: {err}");
        }

        let (res, buf) = api_call_result.unwrap();

        let posts: std::result::Result<Vec<Post>, serde_json::Error> =
            serde_json::from_reader(buf.reader());

        match posts {
            Ok(posts) => {
                println!("posts: {posts:#?}");
            }
            Err(err) => {
                println!("error: {err}");
            }
        }
        assert_eq!(res.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_by_query() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let query_map = HashMap::from([("id", RequestValue::String("1"))]);

        let api_call_result = api.get("/comments", Some(query_map), None).await;

        if let Err(err) = api_call_result {
            panic!("error: {err}");
        }

        let (res, buf) = api_call_result.unwrap();

        let comments: std::result::Result<Vec<Comment>, serde_json::Error> =
            serde_json::from_reader(buf.reader());

        match comments {
            Ok(comments) => {
                println!("comments: {comments:#?}");
                assert_eq!(comments.len(), 1);
            }
            Err(err) => {
                panic!("error: {err}");
            }
        }

        assert_eq!(res.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_post() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", RequestValue::Int(1)),
            ("title", RequestValue::String("Test")),
            ("body", RequestValue::String("Some test to do")),
            ("userId", RequestValue::Int(1)),
        ]);

        let api_call_results = api.post("/posts", Some(body_map), None).await;

        if let Err(err) = api_call_results {
            panic!("error: {err}");
        }

        let (res, buf) = api_call_results.unwrap();

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(buf.reader());

        match posts {
            Ok(posts) => {
                println!("posts: {posts:#?}");
            }
            Err(err) => {
                panic!("error: {err}");
            }
        }

        assert_eq!(res.status, StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_put() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", RequestValue::Int(1)),
            ("title", RequestValue::String("Test")),
            ("body", RequestValue::String("Some test to do")),
            ("userId", RequestValue::Int(1)),
        ]);

        let api_call_results = api.put("/posts/1", Some(body_map), None).await;

        if let Err(err) = api_call_results {
            panic!("error: {err}");
        }

        let (res, buf) = api_call_results.unwrap();

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(buf.reader());

        match posts {
            Ok(posts) => {
                println!("posts: {posts:#?}");
            }
            Err(err) => {
                panic!("error: {err}");
            }
        }

        assert_eq!(res.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_patch() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let body_map = HashMap::from([
            ("id", RequestValue::Int(1)),
            ("title", RequestValue::String("Test")),
            ("body", RequestValue::String("Some test to do")),
            ("userId", RequestValue::String("1")),
        ]);

        let api_call_results = api.patch("/posts/1", Some(body_map), None).await;

        if let Err(err) = api_call_results {
            panic!("error: {err}");
        }

        let (res, buf) = api_call_results.unwrap();

        let posts: std::result::Result<Post, serde_json::Error> =
            serde_json::from_reader(buf.reader());

        match posts {
            Ok(posts) => {
                println!("posts: {posts:#?}");
            }
            Err(err) => {
                panic!("error: {err}");
            }
        }

        assert_eq!(res.status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete() {
        let api = Deboa::new("https://jsonplaceholder.typicode.com", None);

        let api_call_results = api.delete("/posts/1", None).await;

        if let Err(err) = api_call_results {
            panic!("error: {err}");
        }

        let (res, _buf) = api_call_results.unwrap();

        assert_eq!(res.status, StatusCode::OK);
    }
}

/*
pub enum DeboaParams {
    Query(HashMap<&'static str, RequestValue>),
    Body(),
}
*/
#[derive(Debug, Serialize, Deserialize, strum_macros::Display)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    OPTIONS,
    TRACE,
    HEAD,
    CONNECT,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RequestValue {
    Int(i32),
    String(&'static str),
}

impl From<RequestValue> for String {
    fn from(value: RequestValue) -> Self {
        match value {
            RequestValue::Int(number) => number.to_string(),
            RequestValue::String(text) => text.to_owned(),
        }
    }
}

impl Display for RequestValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestValue::Int(number) => write!(f, "{number}"),
            RequestValue::String(text) => write!(f, "{text}"),
        }
    }
}

/*
impl ToString for RequestValue {
    fn to_string(&self) -> String {
        match self {
            RequestValue::Int(number) => number.to_string(),
            RequestValue::String(text) => text.to_string(),
        }
    }
}
*/

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
