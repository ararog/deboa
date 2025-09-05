use std::collections::HashMap;
use url::Url;

use crate::{
    client::conn::http2::{BaseHttp2Connection, Http2Connection},
    errors::DeboaError,
    runtimes::tokio::http2::DeboaHttp2Connection,
};

pub struct Http2ConnectionPool {
    #[allow(dead_code)]
    connections: HashMap<Url, BaseHttp2Connection>,
}

impl Http2ConnectionPool {
    pub fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    pub async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttp2Connection, DeboaError> {
        let connection = DeboaHttp2Connection::connect(url.clone()).await?;
        if self.connections.contains_key(url) {
            return Ok(self.connections.get_mut(url).unwrap());
        }

        self.connections.insert(url.clone(), connection);
        Ok(self.connections.get_mut(url).unwrap())
    }
}
