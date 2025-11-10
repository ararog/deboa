use std::path::Path;

use bytes::{Bytes, BytesMut};
use indexmap::IndexMap;
use rand::distr::{Alphanumeric, SampleString};
use urlencoding::encode;

const CRLF: &[u8] = b"\r\n";
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
    /// * `Bytes` - The encoded form.
    ///
    fn build(self) -> Bytes;
}

/// Enum that represents the form.
///
/// # Variants
///
/// * `EncodedForm` - The encoded form.
/// * `MultiPartForm` - The multi part form.
#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub struct EncodedForm {
    fields: IndexMap<String, String>,
}

/// Implement the builder pattern for EncodedForm.
/// 
/// # Returns
///
/// * `Self` - The encoded form.
///
/// # Examples
///
/// ```compile_fail
/// use deboa::form::MultiPartForm;
///
/// let mut client = Deboa::new();
/// let mut form = MultiPartForm::builder();
/// form.field("name", "deboa");
/// form.field("version", "0.0.1");
/// 
/// let request = DeboaRequest::post("https://example.com/register")?
///     .form(form.into())
///     .build()?;
///
/// let mut response = client.execute(request).await?;
/// ```
impl EncodedForm {
    /// Create a new encoded form.
    ///
    /// # Returns
    ///
    /// * `Self` - The encoded form.
    ///
    pub fn builder() -> Self {
        Self {
            fields: IndexMap::new(),
        }
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

    fn build(self) -> Bytes {
        self.fields
            .iter()
            .map(|(key, value)| format!("{}={}", key, encode(value)))
            .collect::<Vec<String>>()
            .join("&")
            .into_bytes()
            .into()
    }
}

#[derive(Debug, Clone)]
pub struct MultiPartForm {
    fields: IndexMap<String, String>,
    boundary: String,
}

/// Implement the builder pattern for MultiPartForm.
/// 
/// # Returns
///
/// * `Self` - The multi part form.
///
/// # Examples
///
/// ```compile_fail
/// use deboa::form::MultiPartForm;
///
/// let mut client = Deboa::new();
/// let mut form = MultiPartForm::builder();
/// form.field("name", "deboa");
/// form.field("version", "0.0.1");
/// 
/// let request = DeboaRequest::post("https://example.com/register")?
///     .form(form.into())
///     .build()?;
///
/// let mut response = client.execute(request).await?;
/// ```
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
            boundary: format!("DeboaFormBdry{}", boundary),
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
    pub fn file<F>(mut self, key: &str, value: F) -> Self
    where
        F: AsRef<std::path::Path>,
    {
        self.fields.insert(
            key.to_string(),
            value.as_ref().to_str().unwrap().to_string(),
        );
        self
    }

    /// Get the boundary of the form.
    ///
    /// # Returns
    ///
    /// * `String` - The boundary.
    ///
    pub fn boundary(&self) -> String {
        self.boundary.to_string()
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

    fn build(self) -> Bytes {
        let mut form = BytesMut::new();
        let boundary = &self.boundary;
        form.extend_from_slice(b"--");
        form.extend_from_slice(boundary.as_bytes());
        form.extend_from_slice(CRLF);
        for (key, value) in &self.fields {
            if Path::is_file(value.as_ref()) {
                let kind = minimime::lookup_by_filename(value);
                let path = Path::new(value);
                if let Some(kind) = kind {
                    let file_name = path.file_name();
                    let file_content = std::fs::read(value).unwrap();
                    if file_name.is_some() {
                        form.extend_from_slice(
                            &format!(
                                "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"",
                                key,
                                file_name.unwrap().to_str().unwrap()
                            )
                            .into_bytes(),
                        );
                        form.extend_from_slice(CRLF);
                        form.extend_from_slice(
                            &format!("Content-Type: {}\r\n", kind.content_type).into_bytes(),
                        );
                        form.extend_from_slice(CRLF);
                        form.extend_from_slice(&file_content);
                        form.extend_from_slice(CRLF);
                    }
                }
            } else {
                form.extend_from_slice(
                    &format!("Content-Disposition: form-data; name=\"{}\"", key).into_bytes(),
                );
                form.extend_from_slice(CRLF);
                form.extend_from_slice(CRLF);
                form.extend_from_slice(value.as_bytes());
                form.extend_from_slice(CRLF);
            }

            form.extend_from_slice(b"--");
            form.extend_from_slice(boundary.as_bytes());

            if key != self.fields.last().unwrap().0 {
                form.extend_from_slice(CRLF);
            } else {
                form.extend_from_slice(b"--\r\n");
            }
        }

        form.into()
    }
}
