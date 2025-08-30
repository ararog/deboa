use deboa::{Deboa, errors::DeboaError, response};

use crate::io::brotli::{BrotliCompress, BrotliDecompress};

#[tokio::test]
async fn test_brotli_compress() -> Result<(), DeboaError> {
    let mut api: Deboa = Deboa::new("http://localhost:8080")?;
    let body = b"lorem ipsum dolor sit amet consectetur adipiscing elit";
    api.set_body(body.to_vec());

    let compressed = api.compress_body()?;

    let mut response = response::DeboaResponse::new(http::StatusCode::OK, http::HeaderMap::new(), compressed.to_vec());

    response.decompress_body()?;

    println!("original....: {body:?}");
    println!("compressed..: {:?}", compressed.to_vec());
    println!("decompressed: {:?}", response.body());

    Ok(())
}
