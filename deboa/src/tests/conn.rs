use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use caramelo::{
    expect,
    matchers::{eq, truthy},
};
use http::Version;

use crate::{
    TestResult,
    cert::{Certificate, ContentEncoding, Identity},
    conn::ConnectionConfig,
};

pub struct ACertificate {
    data: Vec<u8>,
}

impl ACertificate {
    pub fn new(data: &[u8]) -> Self {
        Self { data: data.into() }
    }
}

impl Certificate for ACertificate {
    fn as_bytes(&self) -> &Vec<u8> {
        &self.data
    }
}

pub struct AIdentity {
    cert: Vec<u8>,
    key: Option<Vec<u8>>,
    enc: Option<ContentEncoding>,
}

impl AIdentity {
    pub fn new(cert: &[u8]) -> Self {
        Self { cert: cert.into(), key: None, enc: Some(ContentEncoding::DER) }
    }
}

impl Identity for AIdentity {
    fn cert(&self) -> &Vec<u8> {
        &self.cert
    }

    fn key(&self) -> Option<&Vec<u8>> {
        self.key.as_ref()
    }

    fn encoding(&self) -> Option<&crate::cert::ContentEncoding> {
        self.enc.as_ref()
    }
}

#[test]
fn test_valid_connection_config_builder() -> TestResult<()> {
    let certificate = ACertificate::new(&[1, 2, 4]);
    let identity = AIdentity::new(&[1, 2, 3]);
    let config = ConnectionConfig::builder()
        .scheme("https")
        .host("localhost")
        .port(80)
        .connection_timeout(Duration::from_secs(20))
        .prior_knowledge(true)
        .protocol_version(Version::HTTP_11)
        .certificate(Some(&certificate))
        .identity(Some(&identity))
        .skip_cert_verification(true)
        .client_bind_addr(IpAddr::V4(Ipv4Addr::LOCALHOST))
        .build();

    expect(config.scheme()).to_be(eq("https"));
    expect(config.host()).to_be(eq("localhost"));
    expect(config.port()).to_be(eq(80));
    expect(config.connection_timeout()).to_be(eq(Duration::from_secs(20)));
    expect(config.prior_knowledge()).to_be(truthy());
    expect(config.protocol_version()).to_be(eq(&Version::HTTP_11));
    expect(config.skip_cert_verification()).to_be(truthy());
    expect(config.client_bind_addr()).to_be(eq(&IpAddr::V4(Ipv4Addr::LOCALHOST)));

    Ok(())
}
