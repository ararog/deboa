use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{ConnectionConfig, ConnectionFactory, DeboaConnection},
};
use deboa::Result;
use std::collections::HashMap;
use time::Duration;

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

impl deboa::conn::HttpConnectionPool for HttpConnectionPool {
    type Identity = DeboaIdentity;
    type Certificate = DeboaCertificate;
    type ConnectionDispather = DeboaConnection;

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
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> Result<&'a mut DeboaConnection> {
        if self.max_idle_connections == 0 {
            self.connections
                .clear();
        }

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
        let connection = ConnectionFactory::create_connection(config.protocol(), config).await?;

        self.connections
            .insert(host.to_string(), connection);
        Ok(self
            .connections
            .get_mut(host)
            .unwrap())
    }
}
