use std::collections::HashMap;

use http::HeaderName;
use hyper::header;

#[derive(Default)]
pub struct DeboaConfig {
    pub headers: Option<HashMap<HeaderName, &'static str>>,
}

impl DeboaConfig {
    pub fn add_header(&mut self, key: HeaderName, value: String) -> &mut Self {
        self.headers.as_mut().unwrap().insert(key, value.leak());
        self
    }

    pub fn remove_header(&mut self, key: &'static str) {
        self.headers.as_mut().unwrap().remove(key);
    }

    pub fn has_header(&self, key: &'static str) -> bool {
        self.headers.as_ref().unwrap().contains_key(key)
    }

    pub fn add_bearer_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Bearer {token}");
        if !self.has_header(header::AUTHORIZATION.as_str()) {
          self.add_header(header::AUTHORIZATION, auth);
        }
        self
    }

    pub fn add_basic_auth(&mut self, token: String) -> &mut Self {
        let auth = format!("Basic {token}");
        if !self.has_header(
          header::AUTHORIZATION.as_str() 
        ) {
          self.add_header(
            header::AUTHORIZATION,
            auth
          );
        }
        self
    }
}
