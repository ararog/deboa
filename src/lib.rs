#![deny(warnings)]
#![warn(rust_2018_idioms)]

#[cfg(any(
    all(feature = "tokio-rt", feature = "smol-rt"),
    all(feature = "tokio-rt", feature = "compio-rt"),
    all(feature = "smol-rt", feature = "compio-rt")
))]
compile_error!("Only one runtime feature can be enabled at a time.");

use anyhow::Result;
use http::{header, HeaderName, HeaderValue};
use http_body_util::BodyExt;
use hyper::Request;
#[cfg(feature = "json")]
use serde::Serialize;
use std::collections::HashMap;
use url::{form_urlencoded, Url};

use crate::{config::DeboaConfig, request::RequestMethod, response::DeboaResponse};

mod config;
mod request;
mod response;
mod runtimes;
mod tests;

pub struct Deboa {
    base_url: &'static str,
    config: Option<DeboaConfig>,
    params: Option<HashMap<&'static str, &'static str>>,
    body: Option<String>,
}

trait DeboaMonitor {
    fn monitor(&self, deboa: &DeboaResponse);
}

trait DeboaRequestTransformer {
    fn transform(&self);
}

trait DeboaResponseTransformer {
    fn transform(&self, deboa: &mut DeboaResponse);
}

impl DeboaMonitor for Deboa {
    fn monitor(&self, deboa: &DeboaResponse) {
        
    }
}

impl DeboaRequestTransformer for Deboa {
    fn transform(&self) {
        
    }
}

impl DeboaResponseTransformer for Deboa {
    fn transform(&self, deboa: &mut DeboaResponse) {
        
    }
}

impl   Deboa {
    pub fn new(base_url: &'static str) -> Self {
        let default_headers: HashMap<HeaderName, String> = HashMap::from([
            (header::ACCEPT, "application/json".to_string()),
            (header::CONTENT_TYPE, "application/json".to_string()),
        ]);

        Deboa {
            base_url,
            config: Some(DeboaConfig {
                headers: default_headers,
            }),
            params: None,
            body: None,
        }
    }

    pub fn set_base_url(&mut self, base_url: &'static str) -> &mut Self {
        self.base_url = base_url;
        self
    }

    #[cfg(feature = "json")]
    pub fn set_json<T: Serialize>(&mut self, data: T) -> &mut Self {
        match serde_json::to_string(&data) {
            Ok(json) => self.body = Some(json),
            Err(_) => self.body = None,
        }
        self
    }

    #[cfg(feature = "xml")]
    pub fn set_xml<T: Serialize>(&mut self, data: T) -> &mut Self {
        match serde_json::to_string(&data) {
            Ok(json) => self.body = Some(json),
            Err(_) => self.body = None,
        }
        self
    }

    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.body = Some(text);
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

        DeboaRequestTransformer::transform(self);

        #[cfg(feature = "tokio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::tokio::get_connection(&url).await?;

            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            });

            sender
        };

        #[cfg(feature = "smol-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::smol::get_connection(&url).await?;

            smol::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            })
            .detach();

            sender
        };

        #[cfg(feature = "compio-rt")]
        let mut sender = {
            let (sender, conn) = runtimes::compio::get_connection(&url).await?;

            compio::runtime::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {err:?}");
                }
            })
            .detach();

            sender
        };

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if let Some(config) = &self.config {
                config
                    .headers
                    .iter()
                    .fold(req_headers, |acc, (key, value)| {
                        acc.insert(key, HeaderValue::from_str(&value).unwrap());
                        acc
                    });
            }
        }

        let req = match &self.body {
            Some(body) => builder.body(body.clone())?,
            None => builder.body(String::new())?,
        };

        let res = sender.send_request(req).await?;

        let mut response = DeboaResponse {
            status: res.status(),
            headers: res.headers().clone(),
            body: Box::new(res.collect().await?.aggregate()),
        };

       DeboaResponseTransformer::transform(self, &mut response);

       DeboaMonitor::monitor(self, &response);

        Ok(response)
    }
}