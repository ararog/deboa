use std::path::Path;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use indexmap::IndexMap;
use rand::distr::{Alphanumeric, SampleString};
use urlencoding::encode;

/// Trait to allow create a form.
pub trait DeboaForm {
    /// Get the content type of the form.
    ///
    /// # Returns
    ///
    /// * `String` - The content type.
    ///
    fn content_type(&self) -> String;
    /// Add a field to the form.
    ///
    /// # Arguments
    ///
    /// * `key` - The key.
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The form.
    ///
    fn field(&mut self, key: &str, value: &str) -> &mut Self;
    /// Build the form.
    ///
    /// # Returns
    ///
    /// * `String` - The encoded form.
    ///
    fn build(&self) -> String;
}

/// Enum that represents the form.
///
/// # Variants
///
/// * `EncodedForm` - The encoded form.
/// * `MultiPartForm` - The multi part form.
pub enum Form {
    EncodedForm(EncodedForm),
    MultiPartForm(MultiPartForm),
}

impl From<EncodedForm> for Form {
    fn from(val: EncodedForm) -> Self {
        Form::EncodedForm(val)
    }
}

impl From<MultiPartForm> for Form {
    fn from(val: MultiPartForm) -> Self {
        Form::MultiPartForm(val)
    }
}

#[derive(Debug)]
pub struct EncodedForm {
    fields: IndexMap<String, String>,
}

impl EncodedForm {
    /// Create a new encoded form.
    ///
    /// # Returns
    ///
    /// * `Self` - The encoded form.
    ///
    pub fn builder() -> Self {
        Self { fields: IndexMap::new() }
    }
}

impl DeboaForm for EncodedForm {
    fn content_type(&self) -> String {
        "application/x-www-form-urlencoded".to_string()
    }

    fn field(&mut self, key: &str, value: &str) -> &mut Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    fn build(&self) -> String {
        self.fields
            .iter()
            .map(|(key, value)| format!("{}={}", key, encode(value)))
            .collect::<Vec<String>>()
            .join("&")
    }
}

#[derive(Debug)]
pub struct MultiPartForm {
    fields: IndexMap<String, String>,
    boundary: String,
}

impl MultiPartForm {
    /// Create a new multi part form.
    ///
    /// # Returns
    ///
    /// * `Self` - The multi part form.
    ///
    pub fn builder() -> Self {
        let boundary = Alphanumeric.sample_string(&mut rand::rng(), 10);
        Self {
            fields: IndexMap::new(),
            boundary: format!("-----DeboaFormBdry{}", boundary),
        }
    }

    /// Add a file to the form.
    ///
    /// # Arguments
    ///
    /// * `key` - The key.
    /// * `value` - The value.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The form.
    ///
    pub fn file<F>(&mut self, key: &str, value: F) -> &mut Self
    where
        F: AsRef<std::path::Path>,
    {
        self.fields.insert(key.to_string(), value.as_ref().to_str().unwrap().to_string());
        self
    }

    /// Get the boundary of the form.
    ///
    /// # Returns
    ///
    /// * `&String` - The boundary.
    ///
    fn boundary(&self) -> &String {
        &self.boundary
    }
}

impl DeboaForm for MultiPartForm {
    fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }

    fn field(&mut self, key: &str, value: &str) -> &mut Self {
        self.fields.insert(key.to_string(), value.to_string());
        self
    }

    fn build(&self) -> String {
        let mut form = String::new();
        let boundary = self.boundary();
        form.push_str("\r\n");
        for (key, value) in &self.fields {
            if Path::is_file(value.as_ref()) {
                let kind = minimime::lookup_by_filename(value);
                let path = Path::new(value);
                if let Some(kind) = kind {
                    let file_name = path.file_name();
                    let file_content = std::fs::read(value).unwrap();
                    if file_name.is_some() {
                        form.push_str(&format!(
                            "Content-Disposition: file; form-data=\"{}\"; filename=\"{}\"",
                            key,
                            file_name.unwrap().to_str().unwrap()
                        ));
                        form.push_str("\r\n");
                        form.push_str(&format!("Content-Type: {}\r\n", kind.content_type));
                        form.push_str("\r\n");
                        if kind.is_binary() {
                            form.push_str(&STANDARD.encode(&file_content));
                        } else {
                            form.push_str(&String::from_utf8_lossy(&file_content));
                        }
                    }
                }
            } else {
                form.push_str(&format!("Content-Disposition: form-data; name=\"{}\"", key));
                form.push_str("\r\n");
                form.push_str("\r\n");
                form.push_str(value);
            }
            form.push_str(boundary);

            if key != self.fields.last().unwrap().0 {
                form.push_str("\r\n");
            } else {
                form.push_str("--");
            }
        }
        form
    }
}
