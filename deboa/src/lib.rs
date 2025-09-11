use std::fmt::Debug;

use bytes::{Buf, Bytes};
use http::{HeaderValue, Request};
use http_body_util::{BodyExt, Full};

use crate::client::conn::http::DeboaHttpConnection;
#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::interceptor::DeboaInterceptor;
use crate::request::DeboaRequest;

use url::Url;

use crate::errors::DeboaError;
use crate::response::DeboaResponse;

pub mod client;
pub mod cookie;
pub mod errors;
pub mod fs;
pub mod interceptor;
pub mod request;
pub mod response;
mod rt;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    #[cfg(feature = "http1")]
    Http1,
    #[cfg(feature = "http2")]
    Http2,
}

pub async fn get(url: &str) -> Result<DeboaResponse, DeboaError> {
    let mut client = Deboa::new();

    let request = DeboaRequest::get(url).build()?;

    client.execute(request).await
}

pub struct Deboa {
    retries: u32,
    connection_timeout: u64,
    request_timeout: u64,
    interceptors: Option<Vec<Box<dyn DeboaInterceptor>>>,
    protocol: HttpVersion,
    #[cfg(feature = "http1")]
    http1_pool: HttpConnectionPool<Http1Request>,
    #[cfg(feature = "http2")]
    http2_pool: HttpConnectionPool<Http2Request>,
}

impl Debug for Deboa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Deboa")
            .field("retries", &self.retries)
            .field("connection_timeout", &self.connection_timeout)
            .field("request_timeout", &self.request_timeout)
            .field("protocol", &self.protocol)
            .finish()
    }
}

impl Deboa {
    /// Allow create a new Deboa instance.
    ///
    /// # Returns
    ///
    /// * `Deboa` - The new Deboa instance.
    ///
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Deboa {
            retries: 0,
            connection_timeout: 0,
            request_timeout: 0,
            interceptors: None,
            protocol: HttpVersion::Http1,
            #[cfg(feature = "http1")]
            http1_pool: HttpConnectionPool::<Http1Request>::new(),
            #[cfg(feature = "http2")]
            http2_pool: HttpConnectionPool::<Http2Request>::new(),
        }
    }

    /// Allow change protocol at any time.
    ///
    /// # Arguments
    ///
    /// * `protocol` - The protocol to be used.
    ///
    pub fn set_protocol(&mut self, protocol: HttpVersion) -> &mut Self {
        self.protocol = protocol;
        self
    }

    /// Allow change request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    pub fn set_retries(&mut self, retries: u32) -> &mut Self {
        self.retries = retries;
        self
    }

    /// Allow change request connection timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    pub fn set_connection_timeout(&mut self, timeout: u64) -> &mut Self {
        self.connection_timeout = timeout;
        self
    }

    /// Allow change request request timeout at any time.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The new timeout.
    ///
    pub fn set_request_timeout(&mut self, timeout: u64) -> &mut Self {
        self.request_timeout = timeout;
        self
    }

    /// Allow add middleware at any time.
    ///
    /// # Arguments
    ///
    /// * `interceptor` - The interceptor to be added.
    ///
    pub fn add_interceptor(&mut self, interceptor: Box<dyn DeboaInterceptor>) -> &mut Self {
        if let Some(interceptors) = &mut self.interceptors {
            interceptors.push(interceptor);
        } else {
            self.interceptors = Some(vec![interceptor]);
        }
        self
    }

    pub async fn execute(&mut self, mut request: DeboaRequest) -> Result<DeboaResponse, DeboaError> {
        if let Some(interceptors) = &self.interceptors {
            interceptors.iter().for_each(|interceptor| {
                interceptor.on_request(&mut request);
            });
        }

        let url = Url::parse(&request.url());
        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }

        let url = url.unwrap();
        let method = request.method();
        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            request.headers().iter().fold(req_headers, |acc, (key, value)| {
                acc.insert(key, HeaderValue::from_str(value).unwrap());
                acc
            });
        }

        let body = request.raw_body();

        let request = builder.body(Full::new(Bytes::from(body.to_vec())));
        if let Err(err) = request {
            return Err(DeboaError::Request {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let request = request.unwrap();

        #[cfg(all(feature = "http1", feature = "http2"))]
        let response = if self.protocol == HttpVersion::Http1 {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(request).await?
        } else {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(request).await?
        };

        #[cfg(all(feature = "http1", not(feature = "http2")))]
        let response = {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(hyper_request).await?
        };

        #[cfg(all(feature = "http2", not(feature = "http1")))]
        let response = {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(hyper_request).await?
        };

        let status_code = response.status();
        let headers = response.headers().clone();

        let result = response.collect().await;
        if let Err(err) = result {
            return Err(DeboaError::ProcessResponse { message: err.to_string() });
        }

        let mut response_body = result.unwrap().aggregate();

        let raw_body = response_body.copy_to_bytes(response_body.remaining()).to_vec();

        let mut response = DeboaResponse::new(status_code, headers, &raw_body);

        if let Some(interceptors) = &self.interceptors {
            interceptors.iter().for_each(|interceptor| interceptor.on_response(&mut response));
        }

        Ok(response)
    }
}
