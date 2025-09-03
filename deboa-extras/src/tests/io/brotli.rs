use deboa::{Deboa, errors::DeboaError};
use http::header;
use httpmock::MockServer;

use crate::{
    io::brotli::BrotliDecompressor,
    tests::types::{BROTLI_COMPRESSED, DECOMPRESSED, format_address},
};

#[tokio::test]
async fn test_brotli_decompress() -> Result<(), DeboaError> {
    let server = MockServer::start();

    let http_mock = server.mock(|when, then| {
        use http::StatusCode;

        when.method(http::Method::GET.as_str()).path("/sometext");
        then.status(StatusCode::OK.into())
            .header(header::CONTENT_ENCODING.as_str(), "br")
            .body(BROTLI_COMPRESSED);
    });

    let mut api = Deboa::new(&format_address(&server))?;

    let body = b"lorem ipsum";
    api.set_raw_body(body);
    api.accept_encoding(vec![Box::new(BrotliDecompressor)]);

    let response = api.get("/sometext").await?;

    http_mock.assert();

    assert_eq!(response.raw_body(), DECOMPRESSED);

    Ok(())
}
