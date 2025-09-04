use std::collections::HashMap;
use url::Url;

use crate::{
    client::conn::http1::{BaseHttp1Connection, Http1Connection},
    errors::DeboaError,
    runtimes::tokio::http1::DeboaHttp1Connection,
};

pub struct Http1ConnectionPool {
    #[allow(dead_code)]
    connections: HashMap<Url, BaseHttp1Connection>,
}

impl Http1ConnectionPool {
    pub fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    pub async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttp1Connection, DeboaError> {
        let connection = DeboaHttp1Connection::connect(url.clone()).await?;
        if self.connections.contains_key(url) {
            return Ok(self.connections.get_mut(url).unwrap());
        }

        self.connections.insert(url.clone(), connection);
        Ok(self.connections.get_mut(url).unwrap())
    }
}
