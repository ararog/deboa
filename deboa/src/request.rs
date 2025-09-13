use std::{collections::HashMap, fmt::Debug, sync::Arc};

use http::{HeaderName, Method, header};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::Serialize;

use crate::{Deboa, client::serde::RequestBody, cookie::DeboaCookie, errors::DeboaError, response::DeboaResponse};

pub struct DeboaRequestBuilder {
    url: String,
    headers: HashMap<::http::HeaderName, String>,
    cookies: Option<HashMap<String, DeboaCookie>>,
    method: http::Method,
    body: Arc<Vec<u8>>,
}

impl DeboaRequestBuilder {
    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `method` - The method.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn method(mut self, method: http::Method) -> Self {
        self.method = method;
        self
    }

    /// Set the body of the request as raw bytes.
    ///
    /// # Arguments
    ///
    /// * `body` - The body.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn raw_body(mut self, body: &[u8]) -> Self {
        self.body = body.to_vec().into();
        self
    }

    /// Set the headers of the request.
    ///
    /// # Arguments
    ///
    /// * `headers` - The headers.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn headers(mut self, headers: HashMap<HeaderName, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Set the cookies of the request.
    ///
    /// # Arguments
    ///
    /// * `cookies` - The cookies.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn cookies(mut self, cookies: HashMap<String, DeboaCookie>) -> Self {
        self.cookies = Some(cookies);
        self
    }

    /// Add a header to the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn add_header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers.insert(key, value.to_string());
        self
    }

    /// Add a cookie to the request.
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn cookie(mut self, cookie: DeboaCookie) -> Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.insert(cookie.name().to_string(), cookie);
        } else {
            self.cookies = Some(HashMap::from([(cookie.name().to_string(), cookie)]));
        }
        self
    }

    /// Set the body of the request as text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn text(mut self, text: &str) -> Self {
        self.body = text.as_bytes().to_vec().into();
        self
    }

    /// Set the body of the request as a type.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The body type.
    /// * `body` - The body.
    ///
    /// # Returns
    ///
    /// * `Result<Self, DeboaError>` - The request builder.
    ///
    pub fn body_as<T: RequestBody, B: Serialize>(mut self, body_type: T, body: B) -> Result<Self, DeboaError> {
        self.body = body_type.serialize(body)?.into();
        Ok(self)
    }

    /// Add bearer auth to the request.
    ///
    /// # Arguments
    ///
    /// * `token` - The token.
    ///
    pub fn add_bearer_auth(mut self, token: &str) -> Self {
        self.headers.insert(header::AUTHORIZATION, format!("Bearer {token}"));
        self
    }

    /// Add basic auth to the request.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn add_basic_auth(mut self, username: &str, password: &str) -> Self {
        self.headers.insert(
            header::AUTHORIZATION,
            format!("Basic {}", STANDARD.encode(format!("{username}:{password}"))),
        );
        self
    }

    /// Build the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest, DeboaError>` - The request.
    ///
    pub fn build(self) -> Result<DeboaRequest, DeboaError> {
        Ok(DeboaRequest {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            method: self.method,
            body: self.body,
        })
    }

    /// Send the request.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse, DeboaError>` - The response.
    ///
    pub async fn send_with<T: AsMut<Deboa>>(self, mut client: T) -> Result<DeboaResponse, DeboaError> {
        client.as_mut().execute(self.build()?).await
    }
}

pub struct DeboaRequest {
    url: String,
    headers: HashMap<::http::HeaderName, String>,
    cookies: Option<HashMap<String, DeboaCookie>>,
    method: http::Method,
    body: Arc<Vec<u8>>,
}

impl Debug for DeboaRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaRequest")
            .field("url", &self.url)
            .field("headers", &self.headers)
            .field("cookies", &self.cookies)
            .field("method", &self.method)
            .field("body", &self.body)
            .finish()
    }
}

impl AsRef<DeboaRequest> for DeboaRequest {
    fn as_ref(&self) -> &DeboaRequest {
        self
    }
}

impl AsMut<DeboaRequest> for DeboaRequest {
    fn as_mut(&mut self) -> &mut DeboaRequest {
        self
    }
}

impl DeboaRequest {
    /// Allow make a request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    /// * `method` - The method to be used.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    fn at(url: &str, method: http::Method) -> DeboaRequestBuilder {
        DeboaRequestBuilder {
            url: url.to_string(),
            headers: HashMap::new(),
            cookies: None,
            method,
            body: Arc::new(Vec::new()),
        }
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn from(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::at(url, Method::GET)
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn to(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::at(url, Method::POST)
    }

    /// Allow make a GET request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn get(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::from(url)
    }

    /// Allow make a POST request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn post(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::to(url)
    }

    /// Allow make a PUT request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn put(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::to(url).method(Method::PUT)
    }

    /// Allow make a PATCH request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn patch(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::to(url).method(Method::PATCH)
    }

    /// Allow make a DELETE request.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to be requested.
    ///
    /// # Returns
    ///
    /// * `DeboaRequestBuilder` - The request builder.
    ///
    pub fn delete(url: &str) -> DeboaRequestBuilder {
        DeboaRequest::from(url).method(Method::DELETE)
    }

    /// Allow change request method at any time.
    ///
    /// # Arguments
    ///
    /// * `method` - The new method.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn set_method(&mut self, method: http::Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Get request method at any time.
    ///
    /// # Returns
    ///
    /// * `http::Method` - The method.
    ///
    pub fn method(&self) -> &http::Method {
        &self.method
    }

    /// Allow change request url at any time.
    ///
    /// # Arguments
    ///
    /// * `url` - The new url.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self, DeboaError>` - The request.
    ///
    pub fn set_url(&mut self, url: &str) -> Result<&mut Self, DeboaError> {
        self.url = url.to_string();
        Ok(self)
    }

    /// Allow get request url at any time.
    ///
    /// # Returns
    ///
    /// * `String` - The url.
    ///
    pub fn url(&self) -> String {
        self.url.to_string()
    }

    /// Allow get request headers at any time.
    ///
    /// # Returns
    ///
    /// * `HashMap<HeaderName, String>` - The headers.
    ///
    pub fn headers(&self) -> &HashMap<HeaderName, String> {
        &self.headers
    }

    /// Allow get mutable headers
    ///
    /// # Returns
    ///
    /// * `&mut HashMap<HeaderName, String>` - The headers.
    ///
    pub fn headers_mut(&mut self) -> &mut HashMap<HeaderName, String> {
        &mut self.headers
    }

    /// Allow add header at any time.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key to add.
    /// * `value` - The header value to add.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn add_header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        self.headers.insert(key, value.to_string());

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
    fn has_header(&self, key: &HeaderName) -> bool {
        self.headers.contains_key(key)
    }

    /// Allow add bearer auth at any time.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to be used in the Authorization header.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
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
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn add_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        let auth = format!("Basic {}", STANDARD.encode(format!("{username}:{password}")));
        if !self.has_header(&header::AUTHORIZATION) {
            self.add_header(header::AUTHORIZATION, &auth);
        }
        self
    }

    /// Allow add cookie at any time.
    ///
    /// # Arguments
    ///
    /// * `cookie` - The cookie to be added.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn add_cookie(&mut self, cookie: DeboaCookie) -> &mut Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.insert(cookie.name().to_string(), cookie);
        } else {
            self.cookies = Some(HashMap::from([(cookie.name().to_string(), cookie)]));
        }
        self
    }

    /// Allow remove cookie at any time.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn remove_cookie(&mut self, name: &str) -> &mut Self {
        if let Some(cookies) = &mut self.cookies {
            cookies.remove(name);
        }
        self
    }

    /// Allow check if cookie exists at any time.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name to check.
    ///
    /// # Returns
    ///
    /// * `bool` - True if the cookie exists, false otherwise.
    ///
    pub fn has_cookie(&self, name: &str) -> bool {
        if let Some(cookies) = &self.cookies {
            cookies.contains_key(name)
        } else {
            false
        }
    }

    /// Allow add cookies at any time.
    ///
    /// # Arguments
    ///
    /// * `cookies` - The cookies to be added.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn set_cookies(&mut self, cookies: HashMap<String, DeboaCookie>) -> &mut Self {
        self.cookies = Some(cookies);
        self
    }

    /// Allow set text body at any time.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.body = text.as_bytes().to_vec().into();
        self
    }

    /// Allow set body at any time.
    ///
    /// # Arguments
    ///
    /// * `body` - The body to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
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
    /// # Returns
    ///
    /// * `Result<&mut Self, DeboaError>` - The request.
    ///
    pub fn set_body_as<T: RequestBody, B: Serialize>(&mut self, body_type: T, body: B) -> Result<&mut Self, DeboaError> {
        body_type.register_content_type(self);
        self.body = body_type.serialize(body)?.into();
        Ok(self)
    }
}
