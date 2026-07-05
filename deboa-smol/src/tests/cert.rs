use deboa::cert::{Certificate, ContentEncoding};

use crate::{cert::DeboaCertificate, tests::helpers::CA_CERT};

#[test]
fn test_cert_init() {
    let cert = DeboaCertificate::from_slice(CA_CERT, ContentEncoding::DER);
    assert_eq!(cert.as_bytes(), CA_CERT);
}
