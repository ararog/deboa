//! Client certificate handling for secure connections.
//!
//! This module provides the `ClientCert` struct for working with client certificates
//! in HTTPS connections, enabling mutual TLS (mTLS) authentication.

/// Represents a client certificate and its associated data for mutual TLS authentication.
///
/// `ClientCert` encapsulates the client certificate, its password, and an optional
/// certificate authority (CA) certificate. It's used to authenticate the client
/// to the server during the TLS handshake.
///
/// # Examples
///
/// ```
/// use deboa::cert::ClientCert;
///
/// // Create a new client certificate without a CA
/// let cert = ClientCert::new(
///     "/path/to/cert.p12".to_string(),
///     "cert-password".to_string(),
///     None
/// );
///
/// // Create a client certificate with a CA
/// let cert_with_ca = ClientCert::new(
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
pub struct ClientCert {
    cert: String,
    pw: String,
    ca: Option<String>,
}

impl ClientCert {
    /// Allow create a new ClientCert instance.
    ///
    /// # Arguments
    ///
    /// * `cert` - The client certificate.
    /// * `pw` - The client certificate password.
    /// * `ca` - The client certificate authority.
    ///
    /// # Returns
    ///
    /// * `ClientCert` - The new ClientCert instance.
    ///
    pub fn new(cert: String, pw: String, ca: Option<String>) -> Self {
        ClientCert { cert, pw, ca }
    }

    /// Allow get the client certificate.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate.
    ///
    pub fn cert(&self) -> &str {
        &self.cert
    }

    /// Allow get the client certificate password.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate password.
    ///
    pub fn pw(&self) -> &str {
        &self.pw
    }

    /// Allow get the client certificate authority.
    ///
    /// # Returns
    ///
    /// * `Option<&str>` - The client certificate authority.
    ///
    pub fn ca(&self) -> Option<&str> {
        self.ca.as_deref()
    }
}
