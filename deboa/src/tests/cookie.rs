use cookie::{time::OffsetDateTime, Expiration};

use crate::cookie::DeboaCookie;

#[test]
fn test_new_cookie() {
    let cookie = DeboaCookie::new("test", "test");

    assert_eq!(cookie.name(), "test");
    assert_eq!(cookie.value(), "test");
}

#[test]
fn test_set_expires() {
    let mut cookie = DeboaCookie::new("test", "test");

    let now = OffsetDateTime::now_utc();
    cookie.set_expires(Expiration::from(now));

    assert_eq!(cookie.expires().unwrap().datetime(), Some(now));
}

#[test]
fn test_set_path() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_path("/test");

    assert_eq!(cookie.path().unwrap(), &"/test");
}

#[test]
fn test_set_domain() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_domain("test.com");

    assert_eq!(cookie.domain().unwrap(), &"test.com");
}

#[test]
fn test_set_secure() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_secure(true);

    assert!(cookie.secure().unwrap());
}

#[test]
fn test_set_http_only() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_http_only(true);

    assert!(cookie.http_only().unwrap());
}

#[test]
fn test_parse_from_header() {
    let cookie = DeboaCookie::parse_from_header("test=test");

    let cookie = cookie.unwrap();

    assert_eq!(cookie.name(), "test");
    assert_eq!(cookie.value(), "test");
}
