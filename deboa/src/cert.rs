//! Client certificate handling for secure connections.
//!
//! This module provides the `Identity` struct for working with client certificates
//! in HTTPS connections, enabling mutual TLS (mTLS) authentication.
//!
//! It also provides the `Certificate` struct for working with CA certificates.

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
use async_native_tls::{Certificate as NativeCertificate, Identity as NativeIdentity};
#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
use rustls::pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer};

/// Represents a client certificate and its associated data for mutual TLS authentication.
///
/// `Identity` encapsulates the client certificate, its password.
/// It's used to authenticate the client to the server during the
/// TLS handshake.
///
/// # Examples
///
/// ```
/// use deboa::cert::Identity;
///
/// // Create a new client certificate without a CA
/// let cert = Identity::from_pckcs12(
///     &[1, 2, 3],
///     Some("cert-password".to_string()),
/// );
///
/// // Create a client certificate with a CA
/// let cert_with_ca = Identity::from_pckcs8(
///     &[1, 2, 3],
///     &[4, 5, 6],
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct Identity {
    cert: Vec<u8>,
    key: Option<Vec<u8>>,
    password: Option<String>,
}

#[deprecated(note = "Use `Identity` instead")]
pub type ClientCert = Identity;

impl Identity {
    #[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
    pub fn from_pckcs12(cert: &[u8], password: Option<String>) -> Self {
        Identity { cert: cert.to_vec(), key: None, password }
    }

    pub fn from_pckcs8(cert: &[u8], key: &[u8]) -> Self {
        Identity { cert: cert.to_vec(), key: Some(key.to_vec()), password: None }
    }
}

#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
impl TryFrom<&Identity> for (CertificateDer<'static>, PrivateKeyDer<'static>) {
    type Error = std::io::Error;

    fn try_from(value: &Identity) -> Result<Self, Self::Error> {
        let cert = CertificateDer::from_pem_slice(&value.cert);
        if cert.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid certificate",
            ));
        }

        let key = PrivateKeyDer::from_pem_slice(
            value
                .key
                .as_ref()
                .unwrap(),
        );
        if key.is_err() {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid key"));
        }

        Ok((cert.unwrap(), key.unwrap()))
    }
}

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
impl TryFrom<&Identity> for NativeIdentity {
    type Error = std::io::Error;

    fn try_from(value: &Identity) -> Result<Self, Self::Error> {
        let identity = if let Some(password) = &value.password {
            let identity = NativeIdentity::from_pkcs12(&value.cert, password);
            if identity.is_err() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid certificate",
                ));
            }
            identity.unwrap()
        } else if let Some(key) = &value.key {
            let identity = NativeIdentity::from_pkcs8(&value.cert, key);
            if identity.is_err() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid certificate",
                ));
            }
            identity.unwrap()
        } else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "You need provide a password or a key",
            ));
        };

        Ok(identity)
    }
}

/// Represents a ca certificate.
///
/// # Examples
///
/// ```
/// use deboa::cert::Certificate;
///
/// // Create a client certificate with a CA
/// let cert_with_ca = Certificate::from_path(
///     "/path/to/cert.p12",
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct Certificate {
    data: Vec<u8>,
}

impl Certificate {
    /// Create certificate from slice.
    ///
    /// # Arguments
    ///
    /// * `data` - The client certificate data.
    ///
    /// # Returns
    ///
    /// * `Certificate` - The new Certificate instance.
    ///
    pub fn from_slice(data: &[u8]) -> Self {
        Certificate { data: data.to_vec() }
    }

    /// Create certificate from path.
    ///
    /// # Arguments
    ///
    /// * `path` - The client certificate path.
    ///
    /// # Returns
    ///
    /// * `Result<Certificate, std::io::Error>` - The new Certificate instance.
    ///
    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let data = std::fs::read(path)?;
        Ok(Certificate { data })
    }

    /// Allow get the client certificate path.
    ///
    /// # Returns
    ///
    /// * `&str` - The client certificate path.
    ///
    #[inline]
    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.data
    }
}

#[cfg(any(feature = "tokio-rust-tls", feature = "smol-rust-tls"))]
impl TryFrom<&Certificate> for CertificateDer<'static> {
    type Error = std::io::Error;

    fn try_from(value: &Certificate) -> Result<Self, Self::Error> {
        let cert = CertificateDer::from(
            value
                .as_bytes()
                .to_vec(),
        );
        if cert.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid certificate",
            ));
        }
        Ok(cert.unwrap())
    }
}

#[cfg(any(feature = "tokio-native-tls", feature = "smol-native-tls"))]
impl TryFrom<&Certificate> for NativeCertificate {
    type Error = std::io::Error;

    fn try_from(value: &Certificate) -> Result<Self, Self::Error> {
        let cert = NativeCertificate::from_pem(value.as_bytes());
        if cert.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid certificate",
            ));
        }
        Ok(cert.unwrap())
    }
}
