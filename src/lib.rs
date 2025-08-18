#![deny(warnings)]
#![warn(rust_2018_idioms)]

use anyhow::Result;
use bytes::Buf;
use http::{HeaderMap, HeaderValue, StatusCode};
use http_body_util::BodyExt;
use hyper::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::{form_urlencoded, Url};

mod runtimes;
mod tests;

#[derive(Default)]
pub struct DeboaConfig {
    headers: Option<HashMap<&'static str, &'static str>>,
}

impl DeboaConfig {
    pub fn add_header(&mut self, key: &'static str, value: String) -> &mut Self {
        self.headers.as_mut().unwrap().insert(key, value.leak());
        self
    }

    pub fn remove_header(&mut self, key: &'static str) {
        self.headers.as_mut().unwrap().remove(key);
    }

    pub fn has_header(&self, key: &'static str) -> bool {
        self.headers.as_ref().unwrap().contains_key(key)
    }

    pub fn add_bearer_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Bearer {token}");
        if !self.has_header("Authorization") {
          self.add_header("Authorization", auth);
        }
        self
    }

    pub fn add_basic_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Basic {token}");
        if !self.has_header("Authorization") {
          self.add_header("Authorization", auth);
        }
        self
    }
}

pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Box<dyn Buf>,
}

impl DeboaResponse {
    pub async fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
        let body = self.body.as_mut();
        let json = serde_json::from_reader(body.reader());
        if let Err(err) = json {
            return Err(err.into());
        }

        Ok(json.unwrap())
    }
}

pub struct Deboa {
    base_url: &'static str,
    config: Option<DeboaConfig>,
    params: Option<HashMap<&'static str, &'static str>>,
    body: Option<String>,
}

impl Deboa {
    pub fn new(base_url: &'static str) -> Self {
        let default_headers: HashMap<&'static str, &'static str> = HashMap::from([
            ("Accept", "application/json"),
            ("Content-Type", "application/json"),
        ]);

        Deboa {
            base_url,
            config: Some(DeboaConfig {
                headers: Some(default_headers),
            }),
            params: None,
            body: None,
        }
    }

    pub fn set_base_url(&mut self, base_url: &'static str) -> &mut Self {
        self.base_url = base_url;
        self
    }

    pub fn set_json<T: Serialize>(&mut self, data: T) -> &mut Self {
        match serde_json::to_string(&data) {
            Ok(json) => self.body = Some(json),
            Err(_) => self.body = None,
        }
        self
    }

    pub fn set_config(&mut self, config: Option<DeboaConfig>) -> &mut Self {
        self.config = config;
        self
    }

    pub fn set_query(&mut self, params: Option<HashMap<&'static str, &'static str>>) -> &mut Self {
        self.params = params;
        self
    }

    pub fn add_monitor(&mut self) -> &mut Self {
        self
    }

    pub fn add_request_transformer(&mut self) -> &mut Self {
        self
    }

    pub fn add_response_transformer(&mut self) -> &mut Self {
        self
    }

    pub async fn get(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::GET, path).await
    }

    pub async fn post(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::POST, path).await
    }

    pub async fn put(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::PUT, path).await
    }

    pub async fn patch(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::PATCH, path).await
    }

    pub async fn delete(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::DELETE, path).await
    }

    pub async fn head(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::HEAD, path).await
    }

    pub async fn options(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::OPTIONS, path).await
    }

    pub async fn trace(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::TRACE, path).await
    }

    pub async fn connect(&self, path: &str) -> Result<DeboaResponse> {
        self.any(RequestMethod::CONNECT, path).await
    }

    pub async fn any(&self, method: RequestMethod, path: &str) -> Result<DeboaResponse> {
        let mut url = Url::parse(format!("{}{}", self.base_url, path).as_str()).unwrap();

        if self.params.is_some() && method == RequestMethod::GET {
            let query = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(self.params.as_ref().unwrap())
                .finish();
            url.set_query(Some(&query));
        }

        #[cfg(feature = "tokio-rt")]
        let (mut sender, conn) = runtimes::tokio::get_connection(&url).await?;
        #[cfg(feature = "tokio-rt")]
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {err:?}");
            }
        });

        #[cfg(feature = "smol-rt")]
        let (mut sender, conn) = runtimes::smol::get_connection(&url).await?;
        #[cfg(feature = "smol-rt")]
        smol::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {err:?}");
            }
        })
        .detach();

        #[cfg(feature = "compio-rt")]
        let (mut sender, conn) = runtimes::compio::get_connection(&url).await?;
        #[cfg(feature = "compio-rt")]
        compio::runtime::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {err:?}");
            }
        })
        .detach();

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if let Some(config) = &self.config {
                if let Some(headers) = &config.headers {
                    headers.iter().fold(req_headers, |acc, (key, value)| {
                        acc.insert(*key, HeaderValue::from_static(value));
                        acc
                    });
                }
            }
        }

        let req = match &self.body {
            Some(body) => builder.body(body.clone())?,
            None => builder.body(String::new())?,
        };

        let res = sender.send_request(req).await?;

        let response = DeboaResponse {
            status: res.status(),
            headers: res.headers().clone(),
            body: Box::new(res.collect().await?.aggregate()),
        };

        Ok(response)
    }
}

#[derive(Debug, Serialize, Deserialize, strum_macros::Display, PartialEq)]
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
