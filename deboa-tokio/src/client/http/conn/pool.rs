use crate::{
    cert::{DeboaCertificate, DeboaIdentity},
    client::http::conn::{ConnectionConfig, ConnectionFactory, DeboaConnection},
};
use deboa::{
    dns::DnsResolver,
    errors::{ConnectionError, DeboaError},
    Result,
};
use hashbrown::HashMap;
use std::time::Duration;

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
            keep_alive_duration: Duration::from_mins(5),
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
    type ConnectionCache = HashMap<String, DeboaConnection>;

    fn new(max_idle_connections: u32, keep_alive_duration: Duration) -> Self {
        Self { max_idle_connections, keep_alive_duration, connections: HashMap::new() }
    }

    #[inline]
    fn connections(&self) -> &Self::ConnectionCache {
        &self.connections
    }

    #[inline]
    fn connection_count(&self) -> u32 {
        self.connections
            .len() as u32
    }

    async fn create_connection<'a, D>(
        &mut self,
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
        dns_resolver: &D,
    ) -> Result<&mut DeboaConnection>
    where
        D: DnsResolver,
    {
        let key = format!("{}:{}", config.host(), config.port());
        if self
            .connections
            .contains_key(&key)
        {
            log::debug!("Connection already exists for {}, reusing.", key);
            return Ok(self
                .connections
                .get_mut(&key)
                .unwrap());
        }

        log::debug!("Creating new connection for {}", key);
        let connection = tokio::time::timeout(
            config.connection_timeout(),
            ConnectionFactory::create_connection(config, dns_resolver),
        )
        .await
        .map_err(|_| {
            DeboaError::Connection(ConnectionError::Timeout {
                message: format!(
                    "Connection to {} timed out after {:?}",
                    key,
                    config.connection_timeout()
                ),
            })
        })??;

        self.connections
            .insert(key.clone(), connection);

        Ok(self
            .connections
            .get_mut(&key)
            .unwrap())
    }
}
