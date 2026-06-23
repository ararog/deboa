use crate::{
    cert::{Certificate, ContentEncoding},
    tests::helpers::CA_CERT,
};

#[test]
fn test_cert_init() {
    let cert = Certificate::from_slice(CA_CERT, ContentEncoding::DER);
    assert_eq!(cert.as_bytes(), CA_CERT);
}
