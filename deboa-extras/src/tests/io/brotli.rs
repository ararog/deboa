use deboa::{Deboa, errors::DeboaError, response::DeboaResponse};

use crate::{
    io::brotli::{BrotliCompress, BrotliDecompress},
    tests::types::{COMPRESSED, DECOMPRESSED},
};

#[tokio::test]
async fn test_brotli_compress() -> Result<(), DeboaError> {
    let mut api: Deboa = Deboa::new("http://localhost:8080")?;
    let body = b"lorem ipsum";
    api.set_body(body.to_vec());

    let compressed = api.compress_body()?;

    println!("compressed: {:?}", compressed.to_vec());

    assert_eq!(compressed.to_vec(), COMPRESSED.to_vec());

    Ok(())
}

#[tokio::test]
async fn test_brotli_decompress() -> Result<(), DeboaError> {
    let mut response = DeboaResponse::new(http::StatusCode::OK, http::HeaderMap::new(), COMPRESSED.to_vec());

    response.decompress_body()?;

    assert_eq!(response.body(), DECOMPRESSED);

    Ok(())
}
