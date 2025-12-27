use std::sync::Arc;

use crate::http::sse::request::ServerSentEventBuilder;
use deboa::{request::DeboaRequestBuilder, Result};
use http::{header, HeaderValue, Method};
use mime_typed::{MimeStrExt, TextEventStream};
use url::Url;

#[test]
fn test_sse_request() -> Result<()> {
    let request = DeboaRequestBuilder::sse("https://sse.dev/test")?.build();
    assert_eq!(request.method(), Method::GET);
    assert_eq!(request.url(), Arc::new(Url::parse("https://sse.dev/test").unwrap()));
    assert_eq!(
        request
            .headers()
            .get(header::CONTENT_TYPE),
        Some(HeaderValue::from_static(TextEventStream::MIME_STR)).as_ref()
    );
    Ok(())
}
