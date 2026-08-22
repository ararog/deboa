use deboa::cert::{CertificateExt as _, ContentEncoding};
#[cfg(any(feature = "rust-tls", feature = "native-tls"))]
use deboa_compio::{cert::DeboaCertificate, Client};
use deboa_test_utils::common::helpers::{CA_CERT, SERVER_CERT, SERVER_KEY};
use easyhttpmock_vetis_compio::{
    config::EasyHttpMockConfig,
    server::PortGenerator,
    vetis_adapter::{VetisAdapter, VetisAdapterConfig},
    EasyHttpMock,
};
use http::Version;
use rstest::fixture;
use std::net::IpAddr;

pub(crate) const SKIP_CERT_VERIFICATION: bool = cfg!(feature = "native-tls");

#[fixture]
pub(crate) const fn protocol_version() -> Version {
    #[cfg(feature = "http1")]
    return Version::HTTP_11;
    #[cfg(feature = "http2")]
    return Version::HTTP_2;
    #[cfg(feature = "http3")]
    return Version::HTTP_3;
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
        .certificate(DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER))
        .skip_cert_verification(SKIP_CERT_VERIFICATION)
        .bind_addr(addr)
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
        .build()
}

#[fixture]
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
        .interface(
            interface
                .parse()
                .unwrap(),
        )
        .protos(vec![protocol_version()])
        .port(free_port(&interface))
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
        .interface(
            interface
                .parse()
                .unwrap(),
        )
        .protos(vec![protocol_version()])
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

#[fixture]
pub async fn create_server() -> EasyHttpMock<VetisAdapter> {
    #[cfg(any(feature = "rust-tls", feature = "native-tls"))]
    return tls_mock_server().await;
    #[cfg(not(any(feature = "rust-tls", feature = "native-tls")))]
    return plain_mock_server().await;
}

/// An ephemeral port the OS says is free, rather than a guessed one.
///
/// `with_random_port()` picks `rand::random_range(9000..65535)` and
/// de-duplicates against a process-local set. Under `cargo test` that mostly
/// holds, because every test shares one process and therefore one set. Under
/// `cargo nextest run` — which is what CI uses — each test is its own process,
/// so the set protects nothing and two tests eventually roll the same number.
/// The loser dies with `Address already in use (os error 98)` before its body
/// runs, which is why the failing test is a different one each time.
///
/// Binding port 0 asks the kernel for a port it knows is free. There is still a
/// window between dropping this listener and the server binding, but the kernel
/// will not hand the same ephemeral port to someone else inside it, which is
/// the part guessing cannot promise.
fn free_port(interface: &str) -> u16 {
    let listener = std::net::TcpListener::bind((interface, 0))
        .expect("bind an ephemeral port to ask the OS for a free one");
    listener
        .local_addr()
        .expect("read back the bound port")
        .port()
}
