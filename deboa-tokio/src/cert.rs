//! Client certificate handling for secure connections.
//!
//! This module provides the `Identity` struct for working with client certificates
//! in HTTPS connections, enabling mutual TLS (mTLS) authentication.
//!
//! It also provides the `Certificate` struct for working with CA certificates.

#[cfg(feature = "native-tls")]
use async_native_tls::{Certificate as NativeCertificate, Identity as NativeIdentity};
#[cfg(feature = "rust-tls")]
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

#[derive(Debug, Clone)]
/// Supported encodings for client certificates.
pub enum ContentEncoding {
    /// PEM encoding.
    PEM,
    /// DER encoding.
    DER,
}

/// Represents a client certificate and its associated data for mutual TLS authentication.
///
/// `Identity` encapsulates the client certificate, its password.
/// It's used to authenticate the client to the server during the
/// TLS handshake.
///
/// # Examples
///
/// ```igmore
/// use deboa::cert::Identity;
///
/// // Load a DER encoded PKCS#12 archive from a slice of bytes using a password
/// let cert = Identity::from_pkcs12(
///     &[1, 2, 3],
///     Some("cert-password".to_string()),
/// );
///
/// // Load a DER encoded certificate and key from a slice of bytes
/// let cert_with_ca = Identity::from_pkcs8(
///     &[1, 2, 3],
///     &[4, 5, 6],
///     ContentEncoding::DER,
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct Identity {
    cert: Vec<u8>,
    key: Option<Vec<u8>>,
    #[allow(unused)]
    password: Option<String>,
    #[allow(unused)]
    encoding: Option<ContentEncoding>,
}

/// Deprecated: Use `Identity` instead.
///
/// This type alias is kept for backward compatibility but will be removed in a future version.
#[deprecated(note = "Use `Identity` instead")]
pub type ClientCert = Identity;

impl Identity {
    #[cfg(feature = "native-tls")]
    /// Load a DER encoded PKCS#12 archive from a slice of bytes
    ///
    /// # Arguments
    ///
    /// * `bundle` - The DER encoded PKCS#12 archive.
    /// * `password` - The password for the PKCS#12 archive.
    ///
    /// # Returns
    ///
    /// * `Identity` - The new Identity instance.
    ///
    pub fn from_pkcs12(bundle: &[u8], password: Option<String>) -> Self {
        Identity { cert: bundle.to_vec(), key: None, password, encoding: None }
    }

    #[cfg(feature = "native-tls")]
    pub fn from_pkcs12_file(file: &str, password: Option<String>) -> std::io::Result<Self> {
        let data = std::fs::read(file)?;
        Ok(Identity { cert: data, key: None, password, encoding: None })
    }

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
    pub fn from_pkcs8(cert: &[u8], key: &[u8], encoding: ContentEncoding) -> Self {
        Identity {
            cert: cert.to_vec(),
            key: Some(key.to_vec()),
            password: None,
            encoding: Some(encoding),
        }
    }

    /// Load DER encoded certificate and key from files
    ///
    /// # Arguments
    ///
    /// * `cert` - The path to the DER encoded certificate file.
    /// * `key` - The path to the DER encoded PKCS8 private key file.
    /// * `encoding` - The encoding of the certificate and key.
    ///
    /// # Returns
    ///
    /// * `std::io::Result<Self>` - The new Identity instance.
    ///
    pub fn from_pkcs8_file(
        cert: &str,
        key: &str,
        encoding: ContentEncoding,
    ) -> std::io::Result<Self> {
        let cert = std::fs::read(cert)?;
        let key = std::fs::read(key)?;
        Ok(Identity { cert, key: Some(key), password: None, encoding: Some(encoding) })
    }
}

#[cfg(feature = "rust-tls")]
impl TryFrom<&Identity> for (CertificateDer<'static>, PrivateKeyDer<'static>) {
    type Error = std::io::Error;

    fn try_from(value: &Identity) -> std::result::Result<Self, Self::Error> {
        let cert = value.cert.clone();
        let key = value
            .key
            .as_ref()
            .unwrap()
            .clone();

        let pair = match value.encoding {
            Some(ContentEncoding::DER) => {
                let cert = CertificateDer::from(cert);

                let key = PrivateKeyDer::try_from(key);
                if key.is_err() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Invalid certificate",
                    ));
                }
                (cert, key.unwrap())
            }
            Some(ContentEncoding::PEM) => {
                use rustls_pki_types::pem::PemObject;

                let cert = CertificateDer::from_pem_slice(&cert);
                if let Err(e) = cert {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid certificate: {}", e),
                    ));
                }

                let key = PrivateKeyDer::from_pem_slice(&key);
                if let Err(e) = key {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid certificate: {}", e),
                    ));
                }
                (cert.unwrap(), key.unwrap())
            }
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid certificate",
                ));
            }
        };

        Ok(pair)
    }
}

#[cfg(feature = "native-tls")]
impl TryFrom<&Identity> for NativeIdentity {
    type Error = std::io::Error;

    fn try_from(value: &Identity) -> std::result::Result<Self, Self::Error> {
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
/// use deboa_tokio::cert::{Certificate, ContentEncoding};
///
/// // Load a DER encoded certificate from a file
/// let cert = Certificate::from_file(
///     "/path/to/cert.crt",
///     ContentEncoding::DER,
/// );
///
/// ```
#[derive(Debug, Clone)]
pub struct Certificate {
    data: Vec<u8>,
    encoding: ContentEncoding,
}

impl Certificate {
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
    pub fn from_slice(data: &[u8], encoding: ContentEncoding) -> Self {
        Certificate { data: data.to_vec(), encoding }
    }

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
    pub fn from_file(file: &str, encoding: ContentEncoding) -> std::io::Result<Self> {
        let data = std::fs::read(file)?;
        Ok(Certificate { data, encoding })
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

#[cfg(feature = "rust-tls")]
impl TryFrom<&Certificate> for CertificateDer<'static> {
    type Error = std::io::Error;

    fn try_from(value: &Certificate) -> std::result::Result<Self, Self::Error> {
        let cert = match value.encoding {
            ContentEncoding::DER => CertificateDer::from(
                value
                    .as_bytes()
                    .to_vec(),
            ),
            ContentEncoding::PEM => {
                use rustls_pki_types::pem::PemObject;

                let result = CertificateDer::from_pem_slice(value.as_bytes());
                if let Err(e) = result {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Invalid certificate: {}", e),
                    ));
                }
                result.unwrap()
            }
        };

        Ok(cert)
    }
}

#[cfg(feature = "native-tls")]
impl TryFrom<&Certificate> for NativeCertificate {
    type Error = std::io::Error;

    fn try_from(value: &Certificate) -> std::result::Result<Self, Self::Error> {
        let cert = match value.encoding {
            ContentEncoding::DER => NativeCertificate::from_der(value.as_bytes()),
            ContentEncoding::PEM => NativeCertificate::from_pem(value.as_bytes()),
        };

        if let Err(e) = cert {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid certificate: {}", e),
            ));
        }

        Ok(cert.unwrap())
    }
}
