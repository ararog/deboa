use std::{collections::HashMap};

use urlencoding::encode;

pub struct Form {
    fields: HashMap<String, String>,
}

impl Form {
    pub fn builder() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn field(&mut self, key: String, value: String) -> &mut Self {
        self.fields.insert(key, value);
        self
    }

    pub fn build(self) -> String {
        let mut form = String::new();
        for (key, value) in &self.fields {
            form.push_str(&format!("{}={}&", key, encode(value)))
        }
        form
    }
}


pub struct MultiPartForm {
    fields: HashMap<String, String>,
}

impl MultiPartForm {
    pub fn builder() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    pub fn field(&mut self, key: String, value: String) -> &mut Self {
        self.fields.insert(key, value);
        self
    }

    pub fn build(self) -> String {
        let mut form = String::new();
        for (key, value) in &self.fields {
            form.push_str(&format!("{}={}&", key, encode(value)))
        }
        form
    }
}
