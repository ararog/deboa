use async_trait::async_trait;
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::{
    HttpVersion,
    client::conn::http::{BaseHttpConnection, DeboaConnection},
    errors::DeboaError,
};

pub struct HttpConnectionPool {
    connections: HashMap<String, DeboaConnection>,
}

impl AsMut<HttpConnectionPool> for HttpConnectionPool {
    fn as_mut(&mut self) -> &mut HttpConnectionPool {
        self
    }
}

#[async_trait]
pub trait DeboaHttpConnectionPool {
    fn new() -> Self;

    fn connections(&self) -> &HashMap<String, DeboaConnection>;

    async fn create_connection<'a>(&'a mut self, url: &Url, protocol: &HttpVersion) -> Result<&'a mut DeboaConnection, DeboaError>;
}

#[async_trait]
impl DeboaHttpConnectionPool for HttpConnectionPool {
    fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    fn connections(&self) -> &HashMap<String, DeboaConnection> {
        &self.connections
    }

    async fn create_connection(&mut self, url: &Url, protocol: &HttpVersion) -> Result<&mut DeboaConnection, DeboaError> {
        use crate::client::conn::http::DeboaHttpConnection;

        let host = Cow::from(url.host().unwrap().to_string());
        if self.connections.contains_key(&host.to_string()) {
            return Ok(self.connections.get_mut(&host.to_string()).unwrap());
        }

        let connection = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => DeboaConnection::Http1(Box::new(BaseHttpConnection::<Http1Request>::connect(url.clone()).await?)),
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => DeboaConnection::Http2(Box::new(BaseHttpConnection::<Http2Request>::connect(url.clone()).await?)),
        };

        self.connections.insert(host.to_string(), connection);
        Ok(self.connections.get_mut(&host.to_string()).unwrap())
    }
}
