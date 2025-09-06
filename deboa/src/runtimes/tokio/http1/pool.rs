use std::collections::HashMap;
use url::Url;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::DeboaError,
};

pub struct Http1ConnectionPool {
    #[allow(dead_code)]
    connections: HashMap<Url, BaseHttpConnection<Http1Request>>,
}

impl Http1ConnectionPool {
    pub fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    pub async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttpConnection<Http1Request>, DeboaError> {
        let connection = BaseHttpConnection::<Http1Request>::connect(url.clone()).await?;
        if self.connections.contains_key(url) {
            return Ok(self.connections.get_mut(url).unwrap());
        }

        self.connections.insert(url.clone(), connection);
        Ok(self.connections.get_mut(url).unwrap())
    }
}
