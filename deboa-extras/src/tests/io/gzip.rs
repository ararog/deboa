use deboa::{Deboa, errors::DeboaError};
use http::{StatusCode, header};
use httpmock::MockServer;

use crate::{
    io::gzip::GzipDecompressor,
    tests::types::{DECOMPRESSED, GZIP_COMPRESSED, format_address},
};

#[tokio::test]
async fn test_gzip() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(http::Method::GET.as_str()).path("/sometext");
        then.status(StatusCode::OK.into())
            .header(header::CONTENT_ENCODING.as_str(), "gzip")
            .body(GZIP_COMPRESSED);
    });

    let mut api = Deboa::new(&format_address(&server))?;

    let body = DECOMPRESSED;
    api.set_raw_body(body.as_ref());
    api.accept_encoding(vec![Box::new(GzipDecompressor)]);

    let response = api.get("/sometext").await?;

    http_mock.assert();

    assert_eq!(response.raw_body(), body);
    Ok(())
}
