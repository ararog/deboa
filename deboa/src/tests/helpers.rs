use std::net::IpAddr;

use deboa_tests::utils::CA_CERT;

use crate::{
    cert::{Certificate, ContentEncoding},
    tests::SKIP_CERT_VERIFICATION,
    Client,
};

pub(crate) fn client_with_cert() -> Client {
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
        .build()
}
