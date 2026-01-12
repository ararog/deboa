use deboa_tests::utils::CA_CERT;

use crate::cert::{Certificate, Identity};

/*
#[test]
fn test_identity_init() {
    let identity = Identity::new_with_pw("cert".into(), Some("pw".into()));
    assert_eq!(identity.cert(), "cert");
    assert_eq!(identity.pw(), Some("pw"));
}

#[test]
fn test_identity_init_with_key() {
    let identity = Identity::new_with_key("cert".into(), "key".into());
    assert_eq!(identity.cert(), "cert");
    assert_eq!(identity.key(), Some("key"));
}
*/

#[test]
fn test_cert_init() {
    let cert = Certificate::from_slice(CA_CERT);
    assert_eq!(cert.as_bytes(), CA_CERT);
}
