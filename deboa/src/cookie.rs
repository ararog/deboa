//! HTTP Cookie handling for the Deboa HTTP client.
//!
//! This module provides the `DeboaCookie` struct for working with HTTP cookies,
//! including parsing from headers and building cookie strings.
//!
//! # Features
//!
//! - Parse cookies from `Set-Cookie` headers
//! - Build cookies with various attributes (expiry, domain, path, etc.)
//! - Convert to/from the `cookie` crate's `Cookie` type
//! - Secure and HttpOnly flags support
//!
//! # Examples
//!
//! ## Creating a new cookie
//!
//! ```compile_fail
//! use deboa::cookie::DeboaCookie;
//! use cookie::time::Duration;
//!
//! // Create a simple session cookie
//! let mut cookie = DeboaCookie::new("session_id", "abc123");
//!
//! // Set additional attributes
//! cookie
//!     .set_path("/")
//!     .set_domain("example.com")
//!     .set_http_only(true)
//!     .set_secure(true);
//! ```
//!
//! ## Parsing from a Set-Cookie header
//!
//! ```compile_fail
//! use deboa::cookie::DeboaCookie;
//!
//! let header = "session_id=abc123; Path=/; Domain=example.com; Secure; HttpOnly";
//! let cookie = DeboaCookie::parse_from_header(header).unwrap();
//!
//! assert_eq!(cookie.name(), "session_id");
//! assert_eq!(cookie.value(), "abc123");
//! assert_eq!(cookie.path(), Some(&"/".to_string()));
//! assert_eq!(cookie.secure(), Some(true));
//! ```

use std::fmt;

use cookie::{Cookie, Expiration};

use crate::{errors::DeboaError, Result};

/// Represents an HTTP cookie with all its attributes.
///
/// `DeboaCookie` provides a builder-style API for creating and manipulating
/// HTTP cookies. It can be converted to/from the `cookie` crate's `Cookie` type
/// and can be parsed from `Set-Cookie` headers.
///
/// # Examples
///
/// ```compile_fail
/// use deboa::cookie::DeboaCookie;
///
/// // Create a new cookie
/// let cookie = DeboaCookie::new("user_id", "12345")
///     .set_path("/")
///     .set_http_only(true);
///
/// // Convert to a cookie string
/// println!("Cookie: {}", cookie);
/// ```
#[derive(Clone, PartialEq)]
pub struct DeboaCookie {
    name: String,
    value: String,
    expires: Option<Expiration>,
    path: Option<String>,
    domain: Option<String>,
    secure: Option<bool>,
    http_only: Option<bool>,
}

impl DeboaCookie {
    /// Create a new cookie.
    ///
    /// # Arguments
    ///
    /// * `name` - The cookie name.
    /// * `value` - The cookie value.
    ///
    /// # Returns
    ///
    /// * `DeboaCookie` - The new cookie.
    ///
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            expires: None,
            path: None,
            domain: None,
            secure: None,
            http_only: None,
        }
    }

    /// Get the cookie name.
    ///
    /// # Returns
    ///
    /// * `&str` - The cookie name.
    ///
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the cookie value.
    ///
    /// # Returns
    ///
    /// * `&str` - The cookie value.
    ///
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Get the cookie expires.
    ///
    /// # Returns
    ///
    /// * `Option<Expiration>` - The cookie expires.
    ///
    pub fn expires(&self) -> Option<Expiration> {
        self.expires
    }

    /// Set the cookie expires.
    ///
    /// # Arguments
    ///
    /// * `expires` - The cookie expires.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_expires(&mut self, expires: Expiration) -> &mut Self {
        self.expires = Some(expires);
        self
    }

    /// Get the cookie path.
    ///
    /// # Returns
    ///
    /// * `Option<&String>` - The cookie path.
    ///
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// Set the cookie path.
    ///
    /// # Arguments
    ///
    /// * `path` - The cookie path.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.path = Some(path.to_string());
        self
    }

    /// Get the cookie domain.
    ///
    /// # Returns
    ///
    /// * `Option<&String>` - The cookie domain.
    ///
    pub fn domain(&self) -> Option<&String> {
        self.domain.as_ref()
    }

    /// Set the cookie domain.
    ///
    /// # Arguments
    ///
    /// * `domain` - The cookie domain.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_domain(&mut self, domain: &str) -> &mut Self {
        self.domain = Some(domain.to_string());
        self
    }

    /// Get the cookie secure.
    ///
    /// # Returns
    ///
    /// * `Option<bool>` - The cookie secure.
    ///
    pub fn secure(&self) -> Option<bool> {
        self.secure
    }

    /// Set the cookie secure.
    ///
    /// # Arguments
    ///
    /// * `secure` - The cookie secure.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = Some(secure);
        self
    }

    /// Get the cookie http only.
    ///
    /// # Returns
    ///
    /// * `Option<bool>` - The cookie http only.
    ///
    pub fn http_only(&self) -> Option<bool> {
        self.http_only
    }

    /// Set the cookie http only.
    ///
    /// # Arguments
    ///
    /// * `http_only` - The cookie http only.
    ///
    /// # Returns
    ///
    /// * `&mut Self` - The cookie.
    ///
    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = Some(http_only);
        self
    }

    /// Parse a cookie from a header.
    ///
    /// # Arguments
    ///
    /// * `header` - The cookie header.
    ///
    /// # Returns
    ///
    /// * `Result<Self>` - The cookie.
    ///
    pub fn parse_from_header(header: &str) -> Result<Self> {
        let cookie = Cookie::parse(header);
        if let Ok(cookie) = cookie {
            Ok(cookie.into())
        } else {
            Err(DeboaError::Cookie {
                message: "Invalid cookie header".to_string(),
            })
        }
    }
}

impl fmt::Display for DeboaCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl fmt::Debug for DeboaCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeboaCookie")
            .field("name", &self.name)
            .field("value", &self.value)
            .field("expires", &self.expires)
            .field("path", &self.path)
            .field("domain", &self.domain)
            .field("secure", &self.secure)
            .field("http_only", &self.http_only)
            .finish()
    }
}

impl From<Cookie<'_>> for DeboaCookie {
    fn from(cookie: Cookie<'_>) -> Self {
        let mut path = None;
        if let Some(path_str) = cookie.path() {
            path = Some(path_str.to_string());
        }

        let mut domain = None;
        if let Some(domain_str) = cookie.domain() {
            domain = Some(domain_str.to_string());
        }

        Self {
            name: cookie.name().to_string(),
            value: cookie.value().to_string(),
            expires: cookie.expires(),
            path,
            domain,
            secure: cookie.secure(),
            http_only: cookie.http_only(),
        }
    }
}

impl From<DeboaCookie> for Cookie<'_> {
    fn from(deboa_cookie: DeboaCookie) -> Self {
        let mut cookie = Self::new(deboa_cookie.name, deboa_cookie.value);
        if let Some(expires) = deboa_cookie.expires {
            cookie.set_expires(expires);
        }
        if let Some(path) = deboa_cookie.path {
            cookie.set_path(path);
        }
        if let Some(domain) = deboa_cookie.domain {
            cookie.set_domain(domain);
        }
        if let Some(secure) = deboa_cookie.secure {
            cookie.set_secure(secure);
        }
        if let Some(http_only) = deboa_cookie.http_only {
            cookie.set_http_only(http_only);
        }
        cookie
    }
}
