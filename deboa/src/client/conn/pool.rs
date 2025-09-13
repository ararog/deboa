use async_trait::async_trait;
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection},
    errors::DeboaError,
};

#[derive(Debug, Clone)]
pub struct HttpConnectionPool<T> {
    connections: HashMap<String, BaseHttpConnection<T>>,
}

#[async_trait]
pub trait DeboaHttpConnectionPool<T> {
    fn new() -> Self;

    fn connections(&self) -> &HashMap<String, BaseHttpConnection<T>>;

    async fn create_connection<'a>(&'a mut self, url: &Url) -> Result<&'a mut BaseHttpConnection<T>, DeboaError>
    where
        T: 'a;
}

#[cfg(feature = "http1")]
#[async_trait]
impl DeboaHttpConnectionPool<Http1Request> for HttpConnectionPool<Http1Request> {
    fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    fn connections(&self) -> &HashMap<String, BaseHttpConnection<Http1Request>> {
        &self.connections
    }

    async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttpConnection<Http1Request>, DeboaError> {
        let host = Cow::from(url.host().unwrap().to_string());
        if self.connections.contains_key(&host.to_string()) {
            return Ok(self.connections.get_mut(&host.to_string()).unwrap());
        }

        let connection = BaseHttpConnection::<Http1Request>::connect(url.clone()).await?;

        self.connections.insert(host.to_string(), connection);
        Ok(self.connections.get_mut(&host.to_string()).unwrap())
    }
}

#[cfg(feature = "http2")]
#[async_trait]
impl DeboaHttpConnectionPool<Http2Request> for HttpConnectionPool<Http2Request> {
    fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    fn connections(&self) -> &HashMap<String, BaseHttpConnection<Http2Request>> {
        &self.connections
    }

    async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttpConnection<Http2Request>, DeboaError> {
        let host = Cow::from(url.host().unwrap().to_string());
        if self.connections.contains_key(&host.to_string()) {
            return Ok(self.connections.get_mut(&host.to_string()).unwrap());
        }

        let connection = BaseHttpConnection::<Http2Request>::connect(url.clone()).await?;

        self.connections.insert(host.to_string(), connection);
        Ok(self.connections.get_mut(&host.to_string()).unwrap())
    }
}
