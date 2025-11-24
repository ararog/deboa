use crate::cert::Identity;

#[test]
fn test_cert_init() {
    let cert = Identity::new("cert".into(), "pw".into(), None);
    assert_eq!(cert.cert(), "cert");
    assert_eq!(cert.pw(), "pw");
    assert_eq!(cert.ca(), None);
}
