#![allow(clippy::upper_case_acronyms)]

use std::{collections::HashMap, sync::Arc};

use bytes::{Buf, Bytes};
use http_body_util::{BodyExt, Full};
use hyper::Request;
use serde::Serialize;

use crate::client::conn::http::DeboaHttpConnection;
#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::HttpVersion;
use crate::client::conn::pool::{DeboaHttpConnectionPool, HttpConnectionPool};
use crate::client::serde::RequestBody;
use crate::{Deboa, fs::io::Decompressor, middleware::DeboaMiddleware};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use http::{HeaderName, HeaderValue, header};
use url::{Url, form_urlencoded};

use crate::errors::DeboaError;
use crate::response::DeboaResponse;

impl Deboa {
    /// Allow create a new Deboa instance.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base url of the api.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use deboa::{Deboa, errors::DeboaError};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), DeboaError> {
    ///   let api = Deboa::new("https://jsonplaceholder.typicode.com")?;
    ///   Ok(())
    /// }
    /// ```
    ///
    pub fn new(base_url: &str) -> Result<Self, DeboaError> {
        let base_url = Url::parse(base_url);
        if let Err(e) = base_url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }

        Ok(Deboa {
            base_url: base_url.unwrap(),
            headers: None,
            query_params: None,
            body: Vec::new().into(),
            retries: 0,
            connection_timeout: 0,
            request_timeout: 0,
            middlewares: None,
            encodings: None,
            protocol: HttpVersion::Http1,
            #[cfg(feature = "http1")]
            http1_pool: HttpConnectionPool::<Http1Request>::new(),
            #[cfg(feature = "http2")]
            http2_pool: HttpConnectionPool::<Http2Request>::new(),
        })
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

    /// Allow change request base url at any time.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The new base url.
    ///
    pub fn set_base_url(&mut self, base_url: &str) -> Result<&mut Self, DeboaError> {
        let url = Url::parse(base_url);
        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }

        self.base_url = url.unwrap();

        Ok(self)
    }

    /// Allow get request base url at any time.
    ///
    /// # Returns
    ///
    /// * `String` - The base url.
    ///
    pub fn base_url(&self) -> String {
        self.base_url.to_string()
    }

    /// Allow add header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to add.
    /// * `value` - The header value to add.
    ///
    pub fn add_header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        if self.headers.is_none() {
            self.headers = Some(HashMap::from([(key, value.to_string())]));
        } else {
            self.headers.as_mut().unwrap().insert(key, value.to_string());
        }

        self
    }

    /// Allow remove header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to remove.
    ///
    pub fn remove_header(&mut self, key: HeaderName) -> &mut Self {
        if let Some(headers) = &mut self.headers {
            headers.remove(&key);
        }
        self
    }

    /// Allow check if header exists at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the header exists, false otherwise.
    ///
    pub fn has_header(&self, key: &HeaderName) -> bool {
        if let Some(headers) = &self.headers {
            headers.contains_key(key)
        } else {
            false
        }
    }

    /// Allow edit header at any time.
    ///
    /// # Arguments
    ///
    /// * `header` - The header to edit.
    /// * `value` - The new header value.
    ///
    pub fn edit_header(&mut self, header: HeaderName, value: &str) -> &mut Self {
        if !self.has_header(&header) {
            self.add_header(header, value);
        } else {
            // We can safely unwrap here, as we have made sure that it exists by the previous if statement.
            let header_value = self.get_mut_header(&header).unwrap();

            *header_value = value.to_string();
        }

        self
    }

    /// Allow get mutable header at any time.
    ///
    /// # Arguments
    ///
    /// * `header` - The header to get.
    ///
    /// # Returns
    ///
    /// * `Option<&mut String>` - The header value.
    ///
    pub fn get_mut_header(&mut self, header: &HeaderName) -> Option<&mut String> {
        if let Some(headers) = &mut self.headers {
            headers.get_mut(header)
        } else {
            None
        }
    }

    /// Allow add bearer auth at any time.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to be used in the Authorization header.
    ///
    pub fn add_bearer_auth(&mut self, token: &str) -> &mut Self {
        let auth = format!("Bearer {token}");
        if !self.has_header(&header::AUTHORIZATION) {
            self.add_header(header::AUTHORIZATION, &auth);
        }
        self
    }

    /// Allow add basic auth at any time.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    pub fn add_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        let auth = format!("Basic {}", STANDARD.encode(format!("{username}:{password}")));
        if !self.has_header(&header::AUTHORIZATION) {
            self.add_header(header::AUTHORIZATION, &auth);
        }
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

    /// Allow set text body at any time.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be set.
    ///
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.body = text.as_bytes().to_vec().into();
        self
    }

    /// Allow add query param at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The query param key.
    /// * `value` - The query param value.
    ///
    pub fn add_query_param(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(query_params) = &mut self.query_params {
            query_params.insert(key.to_string(), value.to_string());
        } else {
            self.query_params = Some(HashMap::from([(key.to_string(), value.to_string())]));
        }
        self
    }

    /// Allow remove query param at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The query param key.
    ///
    pub fn remove_query_param(&mut self, key: &str) -> &mut Self {
        if let Some(query_params) = &mut self.query_params {
            query_params.remove(key);
        }
        self
    }

    /// Allow check if query param exists at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The query param key to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the query param exists, false otherwise.
    ///
    pub fn has_query_param(&self, key: &str) -> bool {
        if let Some(query_params) = &self.query_params {
            query_params.contains_key(key)
        } else {
            false
        }
    }

    /// Allow add query params at any time.
    ///
    /// # Arguments
    ///
    /// * `params` - The query params to be added.
    ///
    pub fn set_query_params(&mut self, params: HashMap<String, String>) -> &mut Self {
        self.query_params = Some(params);
        self
    }

    /// Allow set body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    pub fn set_raw_body(&mut self, body: &[u8]) -> &mut Self {
        self.body = body.to_vec().into();
        self
    }

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&Vec<u8>` - The raw body.
    ///
    pub fn raw_body(&self) -> &Vec<u8> {
        &self.body
    }

    /// Allow set body at any time.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The body type to be set.
    /// * `body` - The body to be set.
    ///
    pub fn set_body_as<T: RequestBody, B: Serialize>(&mut self, body_type: T, body: B) -> Result<&mut Self, DeboaError> {
        body_type.register_content_type(self);
        self.body = body_type.serialize(body)?.into();
        Ok(self)
    }

    /// Allow add middleware at any time.
    ///
    /// # Arguments
    ///
    /// * `middleware` - The middleware to be added.
    ///
    pub fn add_middleware(&mut self, middleware: Box<dyn DeboaMiddleware>) -> &mut Self {
        if let Some(middlewares) = &mut self.middlewares {
            middlewares.push(middleware);
        } else {
            self.middlewares = Some(vec![middleware]);
        }
        self
    }

    /// Allow set accept encoding at any time.
    ///
    /// # Arguments
    ///
    /// * `decompressors` - The decompressors to be set.
    ///
    pub fn accept_encoding(&mut self, decompressors: Vec<Box<dyn Decompressor>>) -> &mut Self {
        let mut encodings = HashMap::new();
        for decompressor in decompressors {
            encodings.insert(decompressor.name(), decompressor);
        }
        let accept_encoding = encodings.keys().map(|key| key.to_string()).collect::<Vec<_>>().join(", ");
        self.edit_header(header::ACCEPT_ENCODING, &accept_encoding);
        self.encodings = Some(encodings);
        self
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn get(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::GET, path).await
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn post(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::POST, path).await
    }

    /// Allow make a PUT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn put(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::PUT, path).await
    }

    /// Allow make a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn patch(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::PATCH, path).await
    }

    /// Allow make a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn delete(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::DELETE, path).await
    }

    /// Allow make a HEAD request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn head(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::HEAD, path).await
    }

    /// Allow make a OPTIONS request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn options(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::OPTIONS, path).await
    }

    /// Allow make a TRACE request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn trace(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::TRACE, path).await
    }

    /// Allow make a CONNECT request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to be requested.
    ///
    pub async fn connect(&mut self, path: &str) -> Result<DeboaResponse, DeboaError> {
        self.any(http::Method::CONNECT, path).await
    }

    /// Allow make a ANY request.
    ///
    /// # Arguments
    ///
    /// * `method` - The method to be requested.
    /// * `path` - The path to be requested.
    ///
    /// Returns
    ///
    /// * `Result<DeboaResponse, DeboaError>` - The response or error.
    ///
    pub async fn any(&mut self, method: http::Method, path: &str) -> Result<DeboaResponse, DeboaError> {
        let url = self.base_url.join(path);

        if let Err(e) = url {
            return Err(DeboaError::UrlParse { message: e.to_string() });
        }

        let mut url = url.unwrap();

        if self.query_params.is_some() && method == http::Method::GET {
            let query = form_urlencoded::Serializer::new(String::new())
                .extend_pairs(self.query_params.as_ref().unwrap())
                .finish();
            url.set_query(Some(&query));
        }

        if let Some(middlewares) = &self.middlewares {
            middlewares.iter().for_each(|middleware| {
                middleware.on_request(self);
            });
        }

        let authority = url.authority();

        let mut builder = Request::builder()
            .uri(url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if let Some(headers) = &self.headers {
                headers.iter().fold(req_headers, |acc, (key, value)| {
                    acc.insert(key, HeaderValue::from_str(value).unwrap());
                    acc
                });
            }
        }

        let body = Arc::clone(&self.body);

        let request = builder.body(Full::new(Bytes::from(body.to_vec())));
        if let Err(err) = request {
            return Err(DeboaError::Request {
                host: url.host().unwrap().to_string(),
                path: url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        #[cfg(all(feature = "http1", feature = "http2"))]
        let response = if self.protocol == HttpVersion::Http1 {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(request.unwrap()).await?
        } else {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(request.unwrap()).await?
        };

        #[cfg(all(feature = "http1", not(feature = "http2")))]
        let response = {
            let conn = self.http1_pool.create_connection(&url).await?;
            conn.send_request(request.unwrap()).await?
        };

        #[cfg(all(feature = "http2", not(feature = "http1")))]
        let response = {
            let conn = self.http2_pool.create_connection(&url).await?;
            conn.send_request(request.unwrap()).await?
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

        if let Some(encodings) = &self.encodings {
            let response_headers = response.headers();
            let content_encoding = response_headers.get(header::CONTENT_ENCODING);
            if let Some(content_encoding) = content_encoding {
                let decompressor = encodings.get(content_encoding.to_str().unwrap());
                if let Some(decompressor) = decompressor {
                    decompressor.decompress_body(&mut response)?;
                }
            }
        }

        if let Some(middlewares) = &self.middlewares {
            middlewares.iter().for_each(|middleware| middleware.on_response(self, &mut response));
        }

        Ok(response)
    }
}
