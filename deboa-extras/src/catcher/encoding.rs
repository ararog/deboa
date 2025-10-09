use async_trait::async_trait;
use deboa::{
    catcher::DeboaCatcher, fs::io::Decompressor, request::DeboaRequest, response::DeboaResponse,
    Result,
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

#[async_trait]
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

    async fn on_response(&self, response: DeboaResponse) -> Result<DeboaResponse> {
        let response_headers = response.headers();
        let content_encoding = response_headers.get(header::CONTENT_ENCODING);
        if let Some(content_encoding) = content_encoding {
            let decompressor = self.accept_encoding.get(content_encoding.to_str().unwrap());
            if let Some(_decompressor) = decompressor {
                //let body = decompressor.decompress_body(&mut response).await?;
                //DeboaResponse::new(response.url(), response.status(), response.headers(), body);
                Ok(response)
            } else {
                Ok(response)
            }
        } else {
            Ok(response)
        }
    }
}
