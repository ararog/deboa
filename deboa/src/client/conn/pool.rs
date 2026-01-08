use async_trait::async_trait;
use std::collections::HashMap;
use time::Duration;

#[cfg(feature = "http1")]
use crate::request::Http1Request;
#[cfg(feature = "http2")]
use crate::request::Http2Request;
#[cfg(feature = "http3-tokio")]
use crate::request::Http3Request;

#[cfg(not(feature = "http3-tokio"))]
use crate::client::conn::tcp::DeboaTcpConnection;
#[cfg(feature = "http3-tokio")]
use crate::client::conn::udp::DeboaUdpConnection;

use crate::{
    cert::{Certificate, Identity},
    client::conn::{BaseHttpConnection, DeboaConnection},
    HttpVersion, Result,
};

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
        is_secure: bool,
        host: &'a str,
        port: u16,
        protocol: &HttpVersion,
        identity: &Option<Identity>,
        certificate: &Option<Certificate>,
        skip_cert_verification: bool,
    ) -> Result<&'a mut DeboaConnection>;
}

impl HttpConnectionPool {
    /// Allow set max idle connections
    ///
    /// # Arguments
    ///
    /// * `max_idle_connections` - The max idle connections.
    ///
    pub fn set_max_idle_connections(&mut self, max_idle_connections: u32) {
        self.max_idle_connections = max_idle_connections;
    }

    /// Allow set keep alive duration
    ///
    /// # Arguments
    ///
    /// * `keep_alive_duration` - The keep alive duration.
    ///
    pub fn set_keep_alive_duration(&mut self, keep_alive_duration: Duration) {
        self.keep_alive_duration = keep_alive_duration;
    }
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

    async fn create_connection<'a>(
        &'a mut self,
        is_secure: bool,
        host: &'a str,
        port: u16,
        protocol: &HttpVersion,
        identity: &Option<Identity>,
        certificate: &Option<Certificate>,
        skip_cert_verification: bool,
    ) -> Result<&'a mut DeboaConnection> {
        if self
            .connections
            .contains_key(host)
        {
            log::debug!("Connection already exists for {}, reusing.", host);
            return Ok(self
                .connections
                .get_mut(host)
                .unwrap());
        }

        log::debug!("Creating new connection for {}", host);
        let connection = match protocol {
            #[cfg(feature = "http1")]
            HttpVersion::Http1 => {
                let connection = BaseHttpConnection::<Http1Request>::connect(
                    is_secure,
                    host,
                    port,
                    identity,
                    certificate,
                    skip_cert_verification,
                )
                .await?;
                DeboaConnection::Http1(Box::new(connection))
            }
            #[cfg(feature = "http2")]
            HttpVersion::Http2 => {
                let connection = BaseHttpConnection::<Http2Request>::connect(
                    is_secure,
                    host,
                    port,
                    identity,
                    certificate,
                    skip_cert_verification,
                )
                .await?;
                DeboaConnection::Http2(Box::new(connection))
            }
            #[cfg(feature = "http3-tokio")]
            HttpVersion::Http3 => {
                let connection = BaseHttpConnection::<Http3Request>::connect(
                    host,
                    port,
                    identity,
                    certificate,
                    skip_cert_verification,
                )
                .await?;
                DeboaConnection::Http3(Box::new(connection))
            }
        };

        self.connections
            .insert(host.to_string(), connection);
        Ok(self
            .connections
            .get_mut(host)
            .unwrap())
    }
}

mod private {
    pub trait DeboaHttpConnectionPoolSealed {}
}

impl private::DeboaHttpConnectionPoolSealed for HttpConnectionPool {}
