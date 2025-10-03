use indexmap::IndexMap;
use urlencoding::encode;

pub struct Form {
    fields: IndexMap<String, String>,
}

impl Form {
    pub fn builder() -> Self {
        Self { fields: IndexMap::new() }
    }

    pub fn field(&mut self, key: &str, value: &str) -> &mut Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&self) -> String {
        self.fields
            .iter()
            .map(|(key, value)| format!("{}={}", key, encode(value)))
            .collect::<Vec<String>>()
            .join("&")
    }
}

pub struct MultiPartForm {
    fields: IndexMap<String, String>,
}

impl MultiPartForm {
    pub fn builder() -> Self {
        Self { fields: IndexMap::new() }
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
