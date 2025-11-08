use crate::cert::ClientCert;

#[test]
fn test_cert_init() {
    let cert = ClientCert::new("cert".to_string(), "pw".to_string(), None);
    assert_eq!(cert.cert(), "cert");
    assert_eq!(cert.pw(), "pw");
    assert_eq!(cert.ca(), None);
}
