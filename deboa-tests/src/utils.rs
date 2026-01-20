use std::future::Future;

use easyhttpmock::{
    config::EasyHttpMockConfig,
    server::{adapters::vetis_adapter::VetisServerAdapter, PortGenerator},
    EasyHttpMock,
};

use vetis::{
    server::{
        config::{SecurityConfig, ServerConfig},
        errors::VetisError,
    },
    RequestType, ResponseType,
};

use url::Url;

pub const CA_CERT: &[u8] = include_bytes!("../certs/ca.der");
pub const CA_CERT_PEM: &[u8] = include_bytes!("../certs/ca.crt");

pub const SERVER_CERT: &[u8] = include_bytes!("../certs/server.der");
pub const SERVER_KEY: &[u8] = include_bytes!("../certs/server.key.der");

pub const SERVER_CERT_PEM: &[u8] = include_bytes!("../certs/server.crt");
pub const SERVER_KEY_PEM: &[u8] = include_bytes!("../certs/server.key");

pub const CLIENT_CERT: &[u8] = include_bytes!("../certs/client.der");
pub const CLIENT_KEY: &[u8] = include_bytes!("../certs/client.key.der");

pub const CLIENT_CERT_PEM: &[u8] = include_bytes!("../certs/client.crt");
pub const CLIENT_KEY_PEM: &[u8] = include_bytes!("../certs/client.key");

pub const CLIENT_P12: &[u8] = include_bytes!("../certs/client.p12");

const TEST_URL: &str = "https://localhost";

pub fn test_url(port: Option<u16>) -> String {
    if let Some(port) = port {
        format!("{}:{}", TEST_URL, port)
    } else {
        TEST_URL.to_string()
    }
}

pub fn fake_url() -> Url {
    Url::parse("http://test.com/get").unwrap()
}

pub fn url_from_string(url: String) -> Url {
    url.parse().unwrap()
}

pub async fn start_mock_server<H, Fut>(handler: H) -> EasyHttpMock<VetisServerAdapter>
where
    H: Fn(RequestType) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<ResponseType, VetisError>> + Send + 'static,
{
    let tls_config = SecurityConfig::builder()
        .cert(SERVER_CERT.to_vec())
        .key(SERVER_KEY.to_vec())
        .build();

    let vetis_config = ServerConfig::builder()
        .security(tls_config)
        .with_random_port()
        .build();

    let config = EasyHttpMockConfig::<VetisServerAdapter>::builder()
        .server_config(vetis_config)
        .build();

    let mut server = EasyHttpMock::new(config);
    #[allow(unused_must_use)]
    let result = server
        .start(handler)
        .await;

    result.unwrap_or_else(|err| {
        panic!("Failed to start mock server: {}", err);
    });

    server
}
