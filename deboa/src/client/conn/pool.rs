use async_trait::async_trait;
use std::{collections::HashMap, sync::Arc};
use time::Duration;
use url::Url;

#[cfg(feature = "http1")]
use crate::client::conn::http::Http1Request;
#[cfg(feature = "http2")]
use crate::client::conn::http::Http2Request;

use crate::{
    cert::ClientCert,
    client::conn::http::{BaseHttpConnection, DeboaConnection, DeboaHttpConnection},
    HttpVersion, Result,
};

#[derive(Debug)]
/// Struct that represents the HTTP connection pool.
///
/// # Fields
///
/// * `connections` - The connections.
pub struct HttpConnectionPool {
    max_idle_connections: u32,
    keep_alive_duration: Duration,
    connections: HashMap<String, DeboaConnection>,
}

impl AsMut<HttpConnectionPool> for HttpConnectionPool {
    fn as_mut(&mut self) -> &mut HttpConnectionPool {
        self
    }
}

impl Default for HttpConnectionPool {
    fn default() -> Self {
        Self {
            max_idle_connections: 5,
            keep_alive_duration: Duration::minutes(5),
            connections: HashMap::new(),
        }
    }
}

#[async_trait]
/// Trait that represents the HTTP connection pool.
pub trait DeboaHttpConnectionPool: private::DeboaHttpConnectionPoolSealed {
    /// Allow create a new connection pool.
    ///
    /// # Returns
    ///
    /// * `HttpConnectionPool` - The new connection pool.
    ///
    fn new(max_idle_connections: u32, keep_alive_duration: Duration) -> Self;

    /// Allow get connections.
    ///
    /// # Returns
    ///
    /// * `&HashMap<String, DeboaConnection>` - The connections.
    ///
    fn connections(&self) -> &HashMap<String, DeboaConnection>;

    /// Returns the number of connections.
    ///
    /// # Returns
    ///
    /// * `u32` - The number of connections.
    ///
    fn connection_count(&self) -> u32;

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
        url: Arc<Url>,
        protocol: &HttpVersion,
        client_cert: &Option<ClientCert>,
    ) -> Result<&'a mut DeboaConnection>;
}

#[async_trait]
impl DeboaHttpConnectionPool for HttpConnectionPool {
    fn new(max_idle_connections: u32, keep_alive_duration: Duration) -> Self {
        Self { max_idle_connections, keep_alive_duration, connections: HashMap::new() }
    }

    #[inline]
    fn connections(&self) -> &HashMap<String, DeboaConnection> {
        &self.connections
    }

    #[inline]
    fn connection_count(&self) -> u32 {
        self.connections
            .len() as u32
    }

    async fn create_connection(
        &mut self,
        url: Arc<Url>,
        protocol: &HttpVersion,
        client_cert: &Option<ClientCert>,
    ) -> Result<&mut DeboaConnection> {
        let mut host = url
            .host_str()
            .unwrap()
            .to_string();
        if url.port().is_some() {
            let port = url.port().unwrap();
            host = format!("{}:{}", host, port);
        } else {
            match url.scheme() {
                "http" | "ws" => host = format!("{}:80", host),
                "https" | "wss" => host = format!("{}:443", host),
                _ => panic!("Unsupported scheme: {}", url.scheme()),
            }
        }

        let host_key = host;
        if self
            .connections
            .contains_key(&host_key)
        {
            log::debug!("Connection already exists for {}, reusing.", host_key);
            return Ok(self
                .connections
                .get_mut(&host_key)
                .unwrap());
        }

        log::debug!("Creating new connection for {}", host_key);
        let connection = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let connection =
                    BaseHttpConnection::<Http1Request>::connect(url, client_cert).await?;
                DeboaConnection::Http1(Box::new(connection))
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let connection =
                    BaseHttpConnection::<Http2Request>::connect(url, client_cert).await?;
                DeboaConnection::Http2(Box::new(connection))
            }
            #[cfg(feature = "http3")]
            HttpVersion::Http3 => {
                let connection =
                    BaseHttpConnection::<Http3Request>::connect(url, client_cert).await?;
                DeboaConnection::Http3(Box::new(connection))
            }
        };

        self.connections
            .insert(host_key.to_string(), connection);
        Ok(self
            .connections
            .get_mut(&host_key)
            .unwrap())
    }
}

mod private {
    pub trait DeboaHttpConnectionPoolSealed {}
}

impl private::DeboaHttpConnectionPoolSealed for HttpConnectionPool {}
