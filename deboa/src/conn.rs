//! Connection module
//!
//! This module provides functionality for managing HTTP connections.
use crate::{
    cert::{Certificate, Identity},
    response::DeboaResponse,
    Result,
};
use http::{Request, Version};
use http_body::Body;
use hyper_body_utils::HttpBody;
use std::{collections::HashMap, future::Future, net::IpAddr};
use time::Duration;

/// Builder for connection configuration.
pub struct ConnectionConfigBuilder<'a, I, C> {
    is_secure: bool,
    ip: IpAddr,
    host: &'a str,
    port: u16,
    protocol_version: Version,
    identity: Option<I>,
    certificate: Option<C>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a, I, C> ConnectionConfigBuilder<'a, I, C>
where
    I: Identity,
    C: Certificate,
{
    /// Create a new connection configuration builder.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            is_secure: false,
            ip: "127.0.0.1"
                .parse::<IpAddr>()
                .unwrap(),
            host: "",
            port: 80,
            protocol_version: Version::HTTP_2,
            identity: None,
            certificate: None,
            skip_cert_verification: false,
            client_bind_addr: "0.0.0.0"
                .parse()
                .unwrap(),
        }
    }

    /// Set whether the connection is secure.
    pub fn is_secure(mut self, is_secure: bool) -> Self {
        self.is_secure = is_secure;
        self
    }

    /// Set the IP address for the connection.
    pub fn ip(mut self, ip: IpAddr) -> Self {
        self.ip = ip;
        self
    }

    /// Set the host for the connection.
    pub fn host(mut self, host: &'a str) -> Self {
        self.host = host;
        self
    }

    /// Set the port for the connection.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the protocol for the connection.
    pub fn protocol_version(mut self, protocol_version: Version) -> Self {
        self.protocol_version = protocol_version;
        self
    }

    /// Set the identity for the connection.
    pub fn identity(mut self, identity: Option<I>) -> Self {
        self.identity = identity;
        self
    }

    /// Set the certificate for the connection.
    pub fn certificate(mut self, certificate: Option<C>) -> Self {
        self.certificate = certificate;
        self
    }

    /// Set whether to skip certificate verification.
    pub fn skip_cert_verification(mut self, skip_cert_verification: bool) -> Self {
        self.skip_cert_verification = skip_cert_verification;
        self
    }

    /// Set the client bind address for the connection.
    pub fn client_bind_addr(mut self, client_bind_addr: IpAddr) -> Self {
        self.client_bind_addr = client_bind_addr;
        self
    }

    /// Build the connection configuration.
    pub fn build(self) -> ConnectionConfig<'a, I, C> {
        ConnectionConfig {
            is_secure: self.is_secure,
            ip: self.ip,
            host: self.host,
            port: self.port,
            protocol_version: self.protocol_version,
            identity: self.identity,
            certificate: self.certificate,
            skip_cert_verification: self.skip_cert_verification,
            client_bind_addr: self.client_bind_addr,
        }
    }
}

/// Connection configuration.
pub struct ConnectionConfig<'a, I, C> {
    is_secure: bool,
    ip: IpAddr,
    host: &'a str,
    port: u16,
    protocol_version: Version,
    identity: Option<I>,
    certificate: Option<C>,
    skip_cert_verification: bool,
    client_bind_addr: IpAddr,
}

impl<'a, I, C> ConnectionConfig<'a, I, C>
where
    I: Identity,
    C: Certificate,
{
    /// Create a new connection configuration builder.
    pub fn builder() -> ConnectionConfigBuilder<'a, I, C> {
        ConnectionConfigBuilder::new()
    }

    /// Get whether the connection is secure.
    pub fn is_secure(&self) -> bool {
        self.is_secure
    }

    /// Get the IP address for the connection.
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    /// Get the host for the connection.
    pub fn host(&self) -> &str {
        self.host
    }

    /// Get the port for the connection.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Get the protocol for the connection.
    pub fn protocol_version(&self) -> &Version {
        &self.protocol_version
    }

    /// Get the identity for the connection.
    pub fn identity(&self) -> &Option<I> {
        &self.identity
    }

    /// Get the certificate for the connection.
    pub fn certificate(&self) -> &Option<C> {
        &self.certificate
    }

    /// Get whether to skip certificate verification.
    pub fn skip_cert_verification(&self) -> bool {
        self.skip_cert_verification
    }

    /// Get the client bind address for the connection.
    pub fn client_bind_addr(&self) -> &IpAddr {
        &self.client_bind_addr
    }
}

/// Trait that represents an HTTP connection.
pub trait HttpConnection {
    /// The sender to use.
    type Sender;

    /// Get the sender.
    fn sender(&mut self) -> &mut Self::Sender;
}

/// Trait that represents the HTTP connection pool.
pub trait HttpConnectionPool {
    /// The identity type.
    type Identity: crate::cert::Identity;
    /// The certificate type.
    type Certificate: crate::cert::Certificate;
    /// The connection dispatcher type.
    type ConnectionDispather: HttpConnectionDispatcher;

    /// Allow create a new connection pool.
    ///
    /// # Returns
    ///
    /// * `HttpConnectionPool` - The new connection pool.
    ///
    fn new(max_idle_connections: u32, keep_alive_duration: Duration) -> Self;

    /// Allow get connections.
    ///
    /// # Returns
    ///
    /// * `&HashMap<String, Self::ConnectionDispather>` - The connections.
    ///
    fn connections(&self) -> &HashMap<String, Self::ConnectionDispather>;

    /// Returns the number of connections.
    ///
    /// # Returns
    ///
    /// * `u32` - The number of connections.
    ///
    fn connection_count(&self) -> u32;

    /// Allow create a new connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The url to connect.
    /// * `protocol` - The protocol to use.
    /// * `retries` - The number of retries.
    ///
    /// # Returns
    ///
    /// * `Result<&mut Self::ConnectionDispather>` - The connection or error.
    ///
    fn create_connection<'a>(
        &'a mut self,
        config: &ConnectionConfig<'a, Self::Identity, Self::Certificate>,
    ) -> impl Future<Output = Result<&'a mut Self::ConnectionDispather>>;
}

/// Trait that represents the HTTP connection dispatcher.
pub trait HttpConnectionDispatcher {
    /// Send a request through the connection.
    ///
    /// # Arguments
    ///
    /// * `request` - The request to send.
    ///
    /// # Returns
    ///
    /// * `Result<DeboaResponse>` - The response from the server.
    fn send_request(
        &mut self,
        request: Request<HttpBody>,
    ) -> impl Future<Output = Result<DeboaResponse>>;
}

/// Trait that represents the HTTP connection.
///
/// # Type Parameters
///
/// * `Sender` - The sender to use.
/// * `ReqBody` - The request body type.
/// * `ResBody` - The response body type.
///
pub trait ProtoConnection {
    /// The request body type.
    type ReqBody: Body + Unpin;
    /// The response body type.
    type ResBody: Body + Unpin;
    /// The connection type.
    type Connection: HttpConnection;
    /// The identity type.
    type Identity: crate::cert::Identity;
    /// The certificate type.
    type Certificate: crate::cert::Certificate;

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `is_secure` - Whether the connection is secure.
    /// * `host` - The host to connect.
    /// * `port` - The port to connect.
    /// * `identity` - The identity to use.
    /// * `certificate` - The certificate to use.
    /// * `skip_cert_verification` - Whether to skip certificate verification.
    ///
    /// # Errors
    ///
    /// * `DeboaError` - If the connection fails.
    ///
    /// # Returns
    ///
    /// * `Result<Self::Connection>` - The connection or error.
    ///
    fn connect(
        config: &ConnectionConfig<Self::Identity, Self::Certificate>,
    ) -> impl Future<Output = Result<Self::Connection>>;

    /// Get connection protocol.
    ///
    /// # Returns
    ///
    /// * `Version` - The connection protocol.
    ///
    fn protocol_version(&self) -> Version;
}

/// Common interface for Plain and TLS stream connections
pub trait StreamConnector {
    /// Connect using ip and port
    fn connect(ip: IpAddr, port: u16);
}
