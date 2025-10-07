use async_trait::async_trait;
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;

#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, DeboaConnection},
    HttpVersion, Result,
};

#[derive(Debug)]
/// Struct that represents the HTTP connection pool.
///
/// # Fields
///
/// * `connections` - The connections.
pub struct HttpConnectionPool {
    connections: HashMap<String, DeboaConnection>,
}

impl AsMut<HttpConnectionPool> for HttpConnectionPool {
    fn as_mut(&mut self) -> &mut HttpConnectionPool {
        self
    }
}

#[async_trait]
/// Trait that represents the HTTP connection pool.
pub trait DeboaHttpConnectionPool {
    /// Allow create a new connection pool.
    ///
    /// # Returns
    ///
    /// * `HttpConnectionPool` - The new connection pool.
    ///
    fn new() -> Self;

    /// Allow get connections.
    ///
    /// # Returns
    ///
    /// * `&HashMap<String, DeboaConnection>` - The connections.
    ///
    fn connections(&self) -> &HashMap<String, DeboaConnection>;

    /// Allow create a new connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    /// * `protocol` - The protocol to use.
    /// * `retries` - The number of retries.
    ///
    /// # Returns
    ///
    /// * `Result<&mut DeboaConnection>` - The connection or error.
    ///
    async fn create_connection<'a>(
        &'a mut self,
        url: &Url,
        protocol: &HttpVersion,
        client_cert: &Option<ClientCert>,
    ) -> Result<&'a mut DeboaConnection>;
}

#[async_trait]
impl DeboaHttpConnectionPool for HttpConnectionPool {
    fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    #[inline]
    fn connections(&self) -> &HashMap<String, DeboaConnection> {
        &self.connections
    }

    async fn create_connection(
        &mut self,
        url: &Url,
        protocol: &HttpVersion,
        client_cert: &Option<ClientCert>,
    ) -> Result<&mut DeboaConnection> {
        use crate::client::conn::http::DeboaHttpConnection;

        let host = Cow::from(url.host().unwrap().to_string());
        let host_key = &host.to_string();
        if self.connections.contains_key(host_key) {
            return Ok(self.connections.get_mut(host_key).unwrap());
        }

        let url = url.clone();
        let connection = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let connection =
                    BaseHttpConnection::<Http1Request>::connect(&url, client_cert).await?;
                DeboaConnection::Http1(Box::new(connection))
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let connection =
                    BaseHttpConnection::<Http2Request>::connect(&url, client_cert).await?;
                DeboaConnection::Http2(Box::new(connection))
            }
        };

        self.connections.insert(host_key.to_string(), connection);
        Ok(self.connections.get_mut(host_key).unwrap())
    }
}
