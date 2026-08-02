#![allow(dead_code)]
#[cfg(feature = "http2")]
use http::Version;
use url::Url;

pub(crate) fn fake_url() -> Url {
    Url::parse("https://httpbin.org/get").unwrap()
}

pub(crate) const fn default_protocol_version() -> Version {
    #[cfg(feature = "http1")]
    return Version::HTTP_11;
    #[cfg(feature = "http2")]
    return Version::HTTP_2;
    #[cfg(feature = "http3")]
    return Version::HTTP_3;
}
