#[derive(Debug, Clone)]
/// Supported encodings for client certificates.
pub enum ContentEncoding {
    /// PEM encoding.
    PEM,
    /// DER encoding.
    DER,
}

/// Identity
pub trait Identity {
    /// Load DER encoded certificate and key from a slice of bytes
    ///
    /// # Arguments
    ///
    /// * `cert` - The DER encoded certificate.
    /// * `key` - The DER encoded PKCS8 private key.
    /// * `encoding` - The encoding of the certificate and key.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    fn from_pkcs8(cert: &[u8], key: &[u8], encoding: ContentEncoding) -> Self;

    /// Load DER encoded certificate and key from files
    ///
    /// # Arguments
    ///
    /// * `cert` - The path to the DER encoded certificate.
    /// * `key` - The path to the DER encoded PKCS8 private key.
    /// * `encoding` - The encoding of the certificate and key.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    ///
    fn from_pkcs8_file(cert: &str, key: &str, encoding: ContentEncoding) -> std::io::Result<Self>
    where
        Self: Sized;
}

/// Certificate
pub trait Certificate {
    /// Create certificate from slice of DER encoded bytes.
    ///
    /// # Arguments
    ///
    /// * `data` - The client certificate data.
    ///
    /// # Returns
    ///
    /// * `Certificate` - The new Certificate instance.
    ///
    fn from_slice(data: &[u8], encoding: ContentEncoding) -> Self;

    /// Create certificate from file of DER encoded file.
    ///
    /// # Arguments
    ///
    /// * `file` - The client certificate file path.
    ///
    /// # Returns
    ///
    /// * `Result<Certificate, std::io::Error>` - The new Certificate instance.
    ///
    fn from_file(file: &str, encoding: ContentEncoding) -> std::io::Result<Self>
    where
        Self: Sized;

    /// Allow get the client certificate path.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate path.
    ///
    fn as_bytes(&self) -> &Vec<u8>;
}
