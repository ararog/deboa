use std::{collections::HashMap, fmt::Debug, sync::Arc};

use async_trait::async_trait;
use http::{
    header::{self, HOST},
    HeaderMap, HeaderName, HeaderValue, Method,
};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde::Serialize;
use url::Url;

use crate::{
    client::serde::RequestBody,
    cookie::DeboaCookie,
    errors::DeboaError,
    form::{DeboaForm, Form},
    response::DeboaResponse,
    url::IntoUrl,
    Deboa, Result,
};

pub trait IntoRequest {
    fn into_request(self) -> Result<DeboaRequest>;
}

impl IntoRequest for DeboaRequest {
    fn into_request(self) -> Result<DeboaRequest> {
        Ok(self)
    }
}

impl IntoRequest for &str {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

impl IntoRequest for String {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

impl IntoRequest for Url {
    fn into_request(self) -> Result<DeboaRequest> {
        DeboaRequest::get(self)?.build()
    }
}

/// Trait to allow make a get request from different types.
#[async_trait]
pub trait Fetch {
    /// Fetch the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    async fn fetch<T: AsMut<Deboa> + Send>(&self, client: T) -> Result<DeboaResponse>;
}

#[async_trait]
impl Fetch for &str {
    async fn fetch<T: AsMut<Deboa> + Send>(&self, client: T) -> Result<DeboaResponse> {
        DeboaRequest::get(*self)?.go(client).await
    }
}

/// A utility function to create a GET request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
#[inline]
pub fn get<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::get(url)
}

/// A utility function to create a POST request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
#[inline]
pub fn post<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::post(url)
}

/// A utility function to create a PUT request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
#[inline]
pub fn put<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::put(url)
}

/// A utility function to create a DELETE request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
#[inline]
pub fn delete<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::delete(url)
}

/// A utility function to create a PATCH request within DeboaRequest.
///
/// # Arguments
///
/// * `url` - The url to connect.
///
/// # Returns
///
/// * `Result<DeboaRequestBuilder>` - The request builder.
///
#[inline]
pub fn patch<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
    DeboaRequest::patch(url)
}

/// Struct that represents the request builder.
///
/// # Fields
///
/// * `url` - The url to connect.
/// * `headers` - The headers to use.
/// * `cookies` - The cookies to use.
/// * `method` - The method to use.
/// * `body` - The body to use.
pub struct DeboaRequestBuilder {
    retries: u32,
    url: Arc<Url>,
    headers: HeaderMap,
    cookies: Option<HashMap<String, DeboaCookie>>,
    method: http::Method,
    body: Arc<[u8]>,
    form: Option<Form>,
}

impl DeboaRequestBuilder {
    /// Allow set request retries at any time.
    ///
    /// # Arguments
    ///
    /// * `retries` - The new retries.
    ///
    pub fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

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
        self.body = body.into();
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
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
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
    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
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

    /// Set multipart form of the request.
    ///
    /// # Arguments
    ///
    /// * `form` - The form.
    ///
    /// # Returns
    ///
    /// * `Self` - The request builder.
    ///
    pub fn form(mut self, form: Form) -> Self {
        self.form = Some(form);
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
        self.body = text.as_bytes().into();
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
    /// * `Result<Self>` - The request builder.
    ///
    pub fn body_as<T: RequestBody, B: Serialize>(mut self, body_type: T, body: B) -> Result<Self> {
        self.body = body_type.serialize(body)?.into();
        Ok(self)
    }

    /// Add bearer auth to the request.
    ///
    /// # Arguments
    ///
    /// * `token` - The token.
    ///
    #[inline]
    pub fn bearer_auth(self, token: &str) -> Self {
        self.header(header::AUTHORIZATION, format!("Bearer {token}").as_str())
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
    pub fn basic_auth(self, username: &str, password: &str) -> Self {
        self.header(
            header::AUTHORIZATION,
            format!(
                "Basic {}",
                STANDARD.encode(format!("{username}:{password}"))
            )
            .as_str(),
        )
    }

    /// Build the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaRequest>` - The request.
    ///
    pub fn build(self) -> Result<DeboaRequest> {
        let mut request = DeboaRequest {
            url: self.url,
            headers: self.headers,
            cookies: self.cookies,
            retries: self.retries,
            method: self.method,
            body: self.body,
        };

        if let Some(form) = self.form {
            request.set_form(form);
        }

        Ok(request)
    }

    /// Send the request.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    pub async fn go<T: AsMut<Deboa>>(self, mut client: T) -> Result<DeboaResponse> {
        client.as_mut().execute(self.build()?).await
    }
}

pub struct DeboaRequest {
    url: Arc<Url>,
    headers: HeaderMap,
    cookies: Option<HashMap<String, DeboaCookie>>,
    retries: u32,
    method: http::Method,
    body: Arc<[u8]>,
}

impl Debug for DeboaRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeboaRequest")
            .field("url", &self.url)
            .field("headers", &self.headers)
            .field("cookies", &self.cookies)
            .field("retries", &self.retries)
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
    pub fn at<T: IntoUrl>(url: T, method: http::Method) -> Result<DeboaRequestBuilder> {
        let parsed_url = url.into_url();
        if let Err(e) = parsed_url {
            return Err(DeboaError::UrlParse {
                message: e.to_string(),
            });
        }

        let url = parsed_url.unwrap();
        let authority = url.authority();
        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, HeaderValue::from_str(authority).unwrap());

        Ok(DeboaRequestBuilder {
            url: url.into(),
            headers,
            cookies: None,
            retries: 0,
            method,
            body: Arc::new([]),
            form: None,
        })
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
    #[inline]
    pub fn from<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
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
    #[inline]
    pub fn to<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
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
    #[inline]
    pub fn get<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::from(url)?.method(Method::GET))
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
    #[inline]
    pub fn post<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::POST))
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
    #[inline]
    pub fn put<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::PUT))
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
    #[inline]
    pub fn patch<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::to(url)?.method(Method::PATCH))
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
    #[inline]
    pub fn delete<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::from(url)?.method(Method::DELETE))
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
    #[inline]
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
    /// * `Result<&mut Self>` - The request.
    ///
    pub fn set_url<T: IntoUrl>(&mut self, url: T) -> Result<&mut Self> {
        let parsed_url = url.into_url();
        if let Err(e) = parsed_url {
            return Err(DeboaError::UrlParse {
                message: e.to_string(),
            });
        }

        let parsed_url = parsed_url.unwrap();
        if self.has_header(&header::HOST) {
            self.headers.remove(&header::HOST);
            self.add_header(HOST, parsed_url.authority());
        }

        self.url = parsed_url.into();
        Ok(self)
    }

    /// Allow get request url at any time.
    ///
    /// # Returns
    ///
    /// * `Url` - The url.
    ///
    #[inline]
    pub fn url(&self) -> Arc<Url> {
        Arc::clone(&self.url)
    }

    /// Allow get retries at any time.
    ///
    /// # Returns
    ///
    /// * `u32` - The retries.
    ///
    #[inline]
    pub fn retries(&self) -> u32 {
        self.retries
    }

    /// Allow get request headers at any time.
    ///
    /// # Returns
    ///
    /// * `HeaderMap` - The headers.
    ///
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Allow get mutable headers
    ///
    /// # Returns
    ///
    /// * `&mut HeaderMap` - The headers.
    ///
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
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
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
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
    #[inline]
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
        let auth = format!(
            "Basic {}",
            STANDARD.encode(format!("{username}:{password}"))
        );
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

    /// Allow get cookies at any time.
    ///
    /// # Returns
    ///
    /// * `Option<&HashMap<String, DeboaCookie>>` - The cookies.
    ///
    pub fn cookies(&self) -> Option<&HashMap<String, DeboaCookie>> {
        self.cookies.as_ref()
    }

    /// Allow set form at any time.
    ///
    /// # Arguments
    ///
    /// * `form` - The form to be set.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The request.
    ///
    pub fn set_form(&mut self, form: Form) -> &mut Self {
        let (content_type, body) = match form {
            Form::EncodedForm(form) => (form.content_type(), form.build()),
            Form::MultiPartForm(form) => (form.content_type(), form.build()),
        };
        self.add_header(header::CONTENT_TYPE, &content_type);
        self.body = body.as_bytes().into();
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
        self.body = text.as_bytes().into();
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
        self.body = body.into();
        self
    }

    /// Allow get raw body at any time.
    ///
    /// # Returns
    ///
    /// * `&Vec<u8>` - The raw body.
    ///
    #[inline]
    pub fn raw_body(&self) -> &[u8] {
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
    /// * `Result<&mut Self>` - The request.
    ///
    pub fn set_body_as<T: RequestBody, B: Serialize>(
        &mut self,
        body_type: T,
        body: B,
    ) -> Result<&mut Self> {
        body_type.register_content_type(self);
        self.body = body_type.serialize(body)?.into();
        Ok(self)
    }
}
