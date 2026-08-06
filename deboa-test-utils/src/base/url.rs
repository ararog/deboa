use deboa::{url::IntoUrl as _, TestResult};

pub fn test_url() -> TestResult<()> {
    let url_str = "http://example.com";
    let url = url_str
        .parse_url()
        .unwrap();
    assert_eq!(url.scheme(), "http");
    assert_eq!(url.host_str(), Some("example.com"));
    Ok(())
}

pub fn test_url_invalid() -> TestResult<()> {
    let url_str = "invalid_url";
    let url = url_str.parse_url();
    assert!(url.is_err());
    Ok(())
}
