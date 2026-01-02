//! Client certificate handling for secure connections.
//!
//! This module provides the `Identity` struct for working with client certificates
//! in HTTPS connections, enabling mutual TLS (mTLS) authentication.

/// Represents a client certificate and its associated data for mutual TLS authentication.
///
/// `Identity` encapsulates the client certificate, its password, and an optional
/// certificate authority (CA) certificate. It's used to authenticate the client
/// to the server during the TLS handshake.
///
/// # Examples
///
/// ```
/// use deboa::cert::Identity;
///
/// // Create a new client certificate without a CA
/// let cert = Identity::new(
///     "/path/to/cert.p12".to_string(),
///     "cert-password".to_string(),
///     None
/// );
///
/// // Create a client certificate with a CA
/// let cert_with_ca = Identity::new(
///     "/path/to/cert.p12".to_string(),
///     "cert-password".to_string(),
///     Some("/path/to/ca.pem".to_string())
/// );
///
/// // Access certificate properties
/// println!("Certificate path: {}", cert.cert());
/// println!("CA path: {:?}", cert.ca());
/// ```
#[derive(Debug, Clone)]
pub struct Identity {
    cert: String,
    key: Option<String>,
    pw: Option<String>,
    ca: Option<String>,
}

#[deprecated(note = "Use `Identity` instead")]
pub type ClientCert = Identity;

impl Identity {
    /// Allow create a new Identity instance.
    ///
    /// # Arguments
    ///
    /// * `cert` - The client certificate.
    /// * `pw` - The client certificate password.
    /// * `ca` - The client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    ///
    #[deprecated(note = "Use `Identity::new_with_pw` instead")]
    pub fn new(cert: String, pw: String, ca: Option<String>) -> Self {
        Identity { cert, key: None, pw: Some(pw), ca }
    }

    /// Create a new Identity instance with optional password.
    ///
    /// # Arguments
    ///
    /// * `cert` - The client certificate.
    /// * `pw` - The client certificate password.
    /// * `ca` - The client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    ///
    pub fn new_with_pw(cert: String, pw: Option<String>, ca: Option<String>) -> Self {
        Identity { cert, key: None, pw, ca }
    }

    /// Create a new Identity instance with key file.
    ///
    /// # Arguments
    ///
    /// * `cert` - The client certificate.
    /// * `key` - The client certificate key file.
    /// * `ca` - The client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    ///
    pub fn new_with_key(cert: String, key: String, ca: Option<String>) -> Self {
        Identity { cert, key: Some(key), pw: None, ca }
    }

    /// Allow get the client certificate.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate.
    ///
    #[inline]
    pub fn cert(&self) -> &str {
        &self.cert
    }

    /// Allow get the client certificate key.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The client certificate key.
    ///
    #[inline]
    pub fn key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    /// Allow get the client certificate password.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The client certificate password.
    ///
    #[inline]
    pub fn pw(&self) -> Option<&str> {
        self.pw.as_deref()
    }

    /// Allow get the client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The client certificate authority.
    ///
    #[inline]
    pub fn ca(&self) -> Option<&str> {
        self.ca.as_deref()
    }
}
