use deboa::cert::{Certificate, ContentEncoding};

use crate::cert::DeboaCertificate;

#[test]
fn test_cert_init() {
    let cert = DeboaCertificate::from_slice(&[1, 2, 3], ContentEncoding::DER);
    assert_eq!(cert.as_bytes(), &[1, 2, 3]);
}
