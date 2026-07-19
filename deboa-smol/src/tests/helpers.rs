#![allow(dead_code)]
use crate::HttpVersion;
use url::Url;

pub(crate) fn fake_url() -> Url {
    Url::parse("https://httpbin.org/get").unwrap()
}

pub(crate) const fn deboa_default_protocol() -> HttpVersion {
    #[cfg(feature = "http1")]
    return HttpVersion::Http1;
    #[cfg(feature = "http2")]
    return HttpVersion::Http2;
    #[cfg(feature = "http3")]
    return HttpVersion::Http3;
}
