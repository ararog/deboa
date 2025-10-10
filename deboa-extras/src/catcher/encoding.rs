use deboa::{
    catcher::DeboaCatcher, errors::DeboaError, fs::io::Decompressor, request::DeboaRequest,
    response::DeboaResponse, Result,
};
use http::header;
use std::collections::HashMap;

pub struct EncodingCatcher<D: Decompressor> {
    pub accept_encoding: HashMap<String, Box<D>>,
}

impl<D: Decompressor> EncodingCatcher<D> {
    pub fn register_decoders(decoders: Vec<D>) -> Self {
        let mut accept_encoding = HashMap::new();
        for decoder in decoders {
            accept_encoding.insert(decoder.name(), Box::new(decoder));
        }
        Self { accept_encoding }
    }
}

#[deboa::async_trait]
impl<D: Decompressor> DeboaCatcher for EncodingCatcher<D> {
    async fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>> {
        let encodings = self
            .accept_encoding
            .values()
            .map(|decoder| decoder.name())
            .collect::<Vec<String>>();

        request.add_header(header::ACCEPT_ENCODING, &encodings.join(","));

        Ok(None)
    }

    async fn on_response(&self, response: &mut DeboaResponse) -> Result<()> {
        let content_encoding = response.headers().get(header::CONTENT_ENCODING);
        if content_encoding.is_none() {
            return Err(DeboaError::Decompress {
                message: "Content encoding not found".to_string(),
            });
        }

        let decompressor = self
            .accept_encoding
            .get(content_encoding.unwrap().to_str().unwrap());
        if decompressor.is_none() {
            return Err(DeboaError::Decompress {
                message: "No decompressor found".to_string(),
            });
        }

        decompressor.unwrap().decompress_body(response).await?;
        Ok(())
    }
}
