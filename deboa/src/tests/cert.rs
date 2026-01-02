use crate::cert::Identity;

#[test]
fn test_cert_init() {
    let cert = Identity::new_with_pw("cert".into(), Some("pw".into()), None);
    assert_eq!(cert.cert(), "cert");
    assert_eq!(cert.pw(), Some("pw"));
    assert_eq!(cert.ca(), None);
}

#[test]
fn test_cert_init_with_key() {
    let cert = Identity::new_with_key("cert".into(), "key".into(), None);
    assert_eq!(cert.cert(), "cert");
    assert_eq!(cert.key(), Some("key"));
    assert_eq!(cert.ca(), None);
}
