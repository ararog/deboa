use std::{collections::HashMap, future::Future};
use time::Duration;

use crate::{
    client::conn::{ConnectionConfig, ConnectionFactory, DeboaConnection},
    Result,
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
    fn create_connection<'a>(
        &'a mut self,
        config: &ConnectionConfig<'a>,
    ) -> impl Future<Output = Result<&'a mut DeboaConnection>>;
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
        config: &ConnectionConfig<'a>,
    ) -> Result<&'a mut DeboaConnection> {
        let host = config.host();
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
        let connection = ConnectionFactory::create_connection(&config.protocol, config).await?;

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
