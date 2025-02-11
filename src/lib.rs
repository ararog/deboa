#![deny(warnings)]
#![warn(rust_2018_idioms)]

use bytes::{buf::Reader, Buf, Bytes};
use http::HeaderValue;
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::TokioIo;
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
        data: Option<HashMap<String, String>>,
        config: Option<DeboaConfig>,
    ) -> Result<Reader<impl Buf>> {
        self.any("POST", path, data, config).await
    }

    pub async fn get(
        self,
        path: &str,
        params: Option<HashMap<String, String>>,
        config: Option<DeboaConfig>,
    ) -> Result<Reader<impl Buf>> {
        self.any("GET", path, params, config).await
    }

    pub async fn put(
        self,
        path: &str,
        data: Option<HashMap<String, String>>,
        config: Option<DeboaConfig>,
    ) -> Result<Reader<impl Buf>> {
        self.any("PUT", path, data, config).await
    }

    pub async fn patch(
        self,
        path: &str,
        data: Option<HashMap<String, String>>,
        config: Option<DeboaConfig>,
    ) -> Result<Reader<impl Buf>> {
        self.any("PATCH", path, data, config).await
    }

    pub async fn delete(self, path: &str) -> Result<Reader<impl Buf>> {
        self.any("DELETE", path, None, None).await
    }

    pub async fn head(self, path: &str) -> Result<Reader<impl Buf>> {
        self.any("HEAD", path, None, None).await
    }

    pub async fn any(
        self,
        method: &str,
        path: &str,
        _params: Option<HashMap<String, String>>,
        config: Option<DeboaConfig>,
    ) -> Result<Reader<impl Buf>> {
        let url = Url::parse(format!("{}{}", self.base_url, path).as_str()).unwrap();

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

        let req = builder.body(Empty::<Bytes>::new())?;

        let res = sender.send_request(req).await?;

        let body = res.collect().await?.aggregate();

        Ok(body.reader())
    }
}
