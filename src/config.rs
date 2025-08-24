use std::collections::HashMap;

use http::HeaderName;

#[derive(Default)]
pub struct DeboaConfig {
    pub headers: HashMap<HeaderName, String>,
}

impl DeboaConfig {
    pub fn add_header(&mut self, key: HeaderName, value: String) -> &mut Self {
        self.headers.insert(key, value);
        self
    }

    pub fn remove_header(&mut self, key: &str) {
        self.headers.remove(key);
    }

    pub fn has_header(&self, header: &HeaderName) -> bool {
        self.headers.contains_key(header)
    }

    /// When adding the header of [`header::AUTHORIZATION`], add the type of authorization to the value itself. ie.: "Bearer {token_here}",.
    pub fn edit_header(&mut self, header: HeaderName, value: String) -> &mut Self {
        if !self.has_header(&header) {
          self.add_header(header, value);
        }
        else {
          // We can safely unwrap here, as we have made sure that it exists by the previous if statement.
          let header_value = self.get_mut_header(&header).unwrap();

          *header_value = value;
        }

        self
    }

    pub fn get_mut_header(&mut self, header: &HeaderName) -> Option<&mut String> {
      self.headers.get_mut(header)
    }
}
