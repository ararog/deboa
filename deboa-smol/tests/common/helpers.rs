use std::net::IpAddr;

use easyhttpmock_vetis_smol::{
    config::EasyHttpMockConfig,
    server::PortGenerator,
    vetis_adapter::{VetisAdapter, VetisAdapterConfig},
    EasyHttpMock, Protocol,
};

use deboa_smol::{
    cert::{Certificate, ContentEncoding},
    Client, HttpVersion,
};

pub(crate) const SKIP_CERT_VERIFICATION: bool = cfg!(feature = "native-tls");

pub const CA_CERT: &[u8] = include_bytes!("../../../certs/ca.der");
// pub const CA_CERT_PEM: &[u8] = include_bytes!("../../../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../../../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../../../certs/server.key.der");

// pub const IP6_SERVER_CERT: &[u8] = include_bytes!("../../../certs/ip6-server.der");
// pub const IP6_SERVER_KEY: &[u8] = include_bytes!("../../../certs/ip6-server.key.der");

// pub const SERVER_CERT_PEM: &[u8] = include_bytes!("../../../certs/server.crt");
// pub const SERVER_KEY_PEM: &[u8] = include_bytes!("../../../certs/server.key");

// pub const CLIENT_CERT: &[u8] = include_bytes!("../../../certs/client.der");
// pub const CLIENT_KEY: &[u8] = include_bytes!("../../../certs/client.key.der");

// pub const CLIENT_CERT_PEM: &[u8] = include_bytes!("../../../certs/client.crt");
// pub const CLIENT_KEY_PEM: &[u8] = include_bytes!("../../../certs/client.key");

// pub const CLIENT_P12: &[u8] = include_bytes!("../../../certs/client.p12");

pub(crate) const fn deboa_default_protocol() -> HttpVersion {
    #[cfg(feature = "http1")]
    return HttpVersion::Http1;
    #[cfg(feature = "http2")]
    return HttpVersion::Http2;
    #[cfg(feature = "http3")]
    return HttpVersion::Http3;
}

pub(crate) const fn vetis_default_protocol() -> Protocol {
    #[cfg(feature = "http1")]
    return Protocol::Http1;
    #[cfg(feature = "http2")]
    return Protocol::Http2;
    #[cfg(feature = "http3")]
    return Protocol::Http3;
}

#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
pub(crate) fn ssl_client() -> Client {
    let interface = std::env::var("INTERFACE").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = interface.parse::<IpAddr>();
    let addr = match addr {
        Ok(addr) => addr,
        Err(e) => panic!("Could not parse IP address: {}", e),
    };

    Client::builder()
        .certificate(Certificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .bind_addr(addr)
        .protocol(deboa_default_protocol())
        .build()
}

#[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
pub(crate) fn plain_client() -> Client {
    let interface = std::env::var("INTERFACE").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = interface.parse::<IpAddr>();
    let addr = match addr {
        Ok(addr) => addr,
        Err(e) => panic!("Could not parse IP address: {}", e),
    };

    Client::builder()
        .bind_addr(addr)
        .protocol(deboa_default_protocol())
        .build()
}

pub(crate) fn create_client() -> Client {
    #[cfg(any(feature = "rust-tls", feature = "native-tls"))]
    return ssl_client();
    #[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
    return plain_client();
}

#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
pub async fn tls_mock_server() -> EasyHttpMock<VetisAdapter> {
    let interface = std::env::var("INTERFACE").unwrap_or_else(|_| "0.0.0.0".to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    let server_cert = SERVER_CERT;
    let server_key = SERVER_KEY;

    let vetis_adapter_config = VetisAdapterConfig::builder()
        .hostname(&hostname)
        .interface(&interface)
        .protocol(vetis_default_protocol())
        .with_random_port()
        .cert(server_cert.to_vec())
        .key(server_key.to_vec())
        .ca(CA_CERT.to_vec())
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let server = EasyHttpMock::new(config);
    let server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    server
}

#[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
pub async fn plain_mock_server() -> EasyHttpMock<VetisAdapter> {
    let interface = std::env::var("INTERFACE").unwrap_or_else(|_| "0.0.0.0".to_string());
    let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string());

    let vetis_adapter_config = VetisAdapterConfig::builder()
        .hostname(&hostname)
        .interface(&interface)
        .protocol(vetis_default_protocol())
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisAdapter>::builder()
        .server_config(vetis_adapter_config)
        .build();

    let server = EasyHttpMock::new(config);
    let server = match server {
        Ok(server) => server,
        Err(err) => {
            panic!("Failed to create mock server: {}", err);
        }
    };

    server
}

pub async fn create_server() -> EasyHttpMock<VetisAdapter> {
    #[cfg(any(feature = "rust-tls", feature = "native-tls"))]
    return tls_mock_server().await;
    #[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
    return plain_mock_server().await;
}
