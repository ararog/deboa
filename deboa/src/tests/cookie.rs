use crate::cookie::DeboaCookie;
use caramelo::{
    expect,
    matchers::{eq, truthy},
};
use cookie::Expiration;
use time::OffsetDateTime;

#[test]
fn test_new_cookie() {
    let cookie = DeboaCookie::new("test", "test");

    expect(cookie.name()).to_be(eq("test"));
    expect(cookie.value()).to_be(eq("test"));
}

#[test]
fn test_set_expires() {
    let mut cookie = DeboaCookie::new("test", "test");

    let now = OffsetDateTime::now_utc();
    cookie.set_expires(Expiration::from(now));

    expect(
        cookie
            .expires()
            .unwrap()
            .datetime(),
    )
    .to_be(eq(Some(now)));
}

#[test]
fn test_set_path() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_path("/test");

    expect(
        cookie
            .path()
            .unwrap(),
    )
    .to_be(eq("/test"));
}

#[test]
fn test_set_domain() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_domain("test.com");

    expect(
        cookie
            .domain()
            .unwrap(),
    )
    .to_be(eq("test.com"));
}

#[test]
fn test_set_secure() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_secure(true);

    assert!(cookie
        .secure()
        .unwrap());
}

#[test]
fn test_set_http_only() {
    let mut cookie = DeboaCookie::new("test", "test");

    cookie.set_http_only(true);

    expect(
        cookie
            .http_only()
            .unwrap(),
    )
    .to_be(truthy());
}

#[test]
fn test_parse_from_header() {
    let cookie = DeboaCookie::parse_from_header("test=test");

    let cookie = cookie.unwrap();

    expect(cookie.name()).to_be(eq("test"));
    expect(cookie.value()).to_be(eq("test"));
}
