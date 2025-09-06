use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;

use crate::{
    client::conn::http::{BaseHttpConnection, DeboaHttpConnection, Http1Request},
    errors::DeboaError,
};

pub struct Http1ConnectionPool {
    #[allow(dead_code)]
    connections: HashMap<String, BaseHttpConnection<Http1Request>>,
}

impl Http1ConnectionPool {
    pub fn new() -> Self {
        Self { connections: HashMap::new() }
    }

    pub async fn create_connection(&mut self, url: &Url) -> Result<&mut BaseHttpConnection<Http1Request>, DeboaError> {
        let host = Cow::from(url.host().unwrap().to_string());
        if self.connections.contains_key(&host.to_string()) {
            return Ok(self.connections.get_mut(&host.to_string()).unwrap());
        }

        let connection = BaseHttpConnection::<Http1Request>::connect(url.clone()).await?;

        self.connections.insert(host.to_string(), connection);
        Ok(self.connections.get_mut(&host.to_string()).unwrap())
    }
}
