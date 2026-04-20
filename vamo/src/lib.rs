#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
use std::sync::Arc;

use crate::resource::{Resource, ResourceMethod};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use deboa::{
    errors::{DeboaError, RequestError},
    request::DeboaRequest,
    response::DeboaResponse,
    serde::RequestBody,
    url::IntoUrl,
    Result,
};
use http::{
    header::{self, CONTENT_TYPE, HOST},
    HeaderMap, HeaderName, HeaderValue, Method,
};
use serde::Serialize;
use url::Url;

pub mod resource;

#[cfg(test)]
mod tests;

/// A builder for HTTP requests.
pub struct Vamo<C> {
    client: C,
    base_url: Url,
    method: Method,
    path: String,
    headers: HeaderMap,
    body: Arc<[u8]>,
}

impl<C> Vamo<C>
where
    C: deboa::HttpClient + Default,
{
    /// Create a new Vamo instance.
    ///
    /// # Arguments
    ///
    /// * `url` - The base URL for the requests.
    ///
    /// # Returns
    ///
    /// * `Result<Vamo>` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    ///
    /// # Panics
    ///
    /// If the URL is invalid, or headers are invalid, the function will panic.
    ///
    pub fn new<U: IntoUrl>(url: U) -> Result<Vamo<C>> {
        let base_url = url.into_url()?;
        let mut headers = HeaderMap::new();
        let host = base_url.host_str();
        if host.is_none() {
            return Err(DeboaError::Request(RequestError::UrlParse {
                message: "Invalid URL: Missing host.".to_string(),
            }));
        }

        let host_header = HeaderValue::from_str(
            base_url
                .host_str()
                .unwrap(),
        );
        if let Err(e) = host_header {
            return Err(DeboaError::Header { message: e.to_string() });
        }

        headers.insert(HOST, host_header.unwrap());

        let content_type_header = HeaderValue::from_str("application/json");
        if let Err(e) = content_type_header {
            return Err(DeboaError::Header { message: e.to_string() });
        }

        headers.insert(CONTENT_TYPE, content_type_header.unwrap());

        Ok(Vamo {
            client: C::default(),
            base_url,
            path: String::new(),
            method: Method::GET,
            headers,
            body: Arc::new([]),
        })
    }

    /// Set the client to be used for requests.
    ///
    /// # Arguments
    ///
    /// * `client` - The client to be used for requests.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    #[inline]
    pub fn client(&mut self, client: C) -> &mut Self {
        self.client = client;
        self
    }

    /// Set a header for the request.
    ///
    /// # Arguments
    ///
    /// * `key` - The header key.
    /// * `value` - The header value.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .header("Content-Type", "application/json")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn header(&mut self, key: HeaderName, value: &str) -> &mut Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    /// Set the body of the request.
    ///
    /// # Arguments
    ///
    /// * `body_type` - The type of the body.
    /// * `body` - The body to be set.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self>` - The builder.
    #[inline]
    pub fn body_as<T: RequestBody, B: Serialize>(
        &mut self,
        body_type: T,
        body: B,
    ) -> Result<&mut Self> {
        self.body = body_type
            .serialize(body)?
            .into();
        Ok(self)
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    #[inline]
    pub fn get(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::GET;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.post("/path").body_as(JSON, body).send().await?;
    /// ```
    #[inline]
    pub fn post(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::POST;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.put("/path/1").body_as(JSON, body).send().await?;
    /// ```
    #[inline]
    pub fn put(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PUT;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.patch("/path/1").body_as(JsonBody, body).send().await?;
    /// ```
    #[inline]
    pub fn patch(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::PATCH;
        self
    }

    /// Set the method of the request.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the request.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.delete("/path/1").send().await?;
    /// ```
    #[inline]
    pub fn delete(&mut self, path: &str) -> &mut Self {
        self.path = path.to_string();
        self.method = Method::DELETE;
        self
    }

    /// Set the bearer token for the request.
    ///
    /// # Arguments
    ///
    /// * `token` - The bearer token.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .bearer_auth("your-token-here")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn bearer_auth(&mut self, token: &str) -> &mut Self {
        self.header(header::AUTHORIZATION, format!("Bearer {token}").as_str());
        self
    }

    /// Set the basic authentication for the request.
    ///
    /// # Arguments
    ///
    /// * `username` - The username.
    /// * `password` - The password.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The builder.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<<deboa_tokio::Client>>::new("https://api.example.com")?;
    /// let response = vamo.get("/api")
    ///    .basic_auth("username", "password")
    ///    .send()
    ///    .await?;
    /// ```
    #[inline]
    pub fn basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            header::AUTHORIZATION,
            format!("Basic {}", STANDARD.encode(format!("{username}:{password}"))).as_str(),
        );
        self
    }

    /// Send the request.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response.
    ///
    /// # Errors
    ///
    /// * `DeboaError` - The error.
    ///
    /// # Examples
    ///
    /// ``` rust, ignore
    /// let mut vamo = Vamo::<deboa_tokio::Client>::new("https://api.example.com")?;
    /// let response = vamo.get("/path").send().await?;
    /// ```
    ///
    /// # Notes
    ///
    /// * The request is sent using the `Deboa` client.
    /// * The response is returned as a `DeboaResponse`.
    ///
    #[inline]
    pub async fn send(&mut self) -> Result<DeboaResponse> {
        let mut base_url = self
            .base_url
            .clone();
        let path_and_query = self
            .path
            .split_once('?');
        let path = if let Some((path, query)) = path_and_query {
            base_url.set_query(Some(query));
            path
        } else {
            &self.path
        };

        let base_path = self.base_url.path();
        if base_path == "/" {
            base_url.set_path(path);
        } else {
            base_url.set_path(&format!("{}{}", base_path, path));
        }

        let request = DeboaRequest::from(base_url.as_str())?
            .method(self.method.clone())
            .headers(self.headers.clone())
            .bytes(&self.body)
            .build()?;

        self.client
            .execute(request)
            .await
    }
}

impl<R: Resource + Serialize, C> ResourceMethod<R> for Vamo<C>
where
    C: deboa::HttpClient,
{
    fn load(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::GET;
        Ok(self)
    }

    fn create(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}", resource.name());
        self.method = Method::POST;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn update(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::PUT;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn edit(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::PATCH;
        self.body = resource
            .body_type()
            .serialize(&resource)?
            .into();
        Ok(self)
    }

    fn remove(&mut self, resource: &mut R) -> Result<&mut Self> {
        self.path = format!("/{}/{}", resource.name(), resource.id());
        self.method = Method::DELETE;
        Ok(self)
    }
}
