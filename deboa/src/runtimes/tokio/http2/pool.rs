use std::collections::HashMap;
use url::Url;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http2Request},
    errors::DeboaError,
};

pub struct Http2ConnectionPool {
    #[allow(dead_code)]
    connections: HashMap<Url, BaseHttpConnection<Http2Request>>,
}

impl Http2ConnectionPool {
    pub fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    pub async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttpConnection<Http2Request>, DeboaError> {
        let connection = BaseHttpConnection::<Http2Request>::connect(url.clone()).await?;
        if self.connections.contains_key(url) {
            return Ok(self.connections.get_mut(url).unwrap());
        }

        self.connections.insert(url.clone(), connection);
        Ok(self.connections.get_mut(url).unwrap())
    }
}
