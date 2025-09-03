use deboa::{Deboa, errors::DeboaError};
use http::StatusCode;
use http::header;
use httpmock::MockServer;

use crate::{
    io::deflate::DeflateDecompressor,
    tests::types::{DECOMPRESSED, DEFLATE_COMPRESSED, format_address},
};

#[tokio::test]
async fn test_deflate_decompress() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        when.method(http::Method::GET.as_str()).path("/sometext");
        then.status(StatusCode::OK.into())
            .header(header::CONTENT_ENCODING.as_str(), "deflate")
            .body(DEFLATE_COMPRESSED);
    });

    let mut api = Deboa::new(&format_address(&server))?;

    let body = DECOMPRESSED;
    api.set_raw_body(body);
    api.accept_encoding(vec![Box::new(DeflateDecompressor)]);

    let response = api.get("/sometext").await?;

    http_mock.assert();

    assert_eq!(response.raw_body(), DECOMPRESSED);

    Ok(())
}
