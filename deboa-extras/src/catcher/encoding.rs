use deboa::{catcher::DeboaCatcher, errors::DeboaError, fs::io::Decompressor, request::DeboaRequest, response::DeboaResponse};
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

impl<D: Decompressor> DeboaCatcher for EncodingCatcher<D> {
    fn on_request(&self, request: &mut DeboaRequest) -> Result<Option<DeboaResponse>, DeboaError> {
        let encodings = self.accept_encoding.values().map(|decoder| decoder.name()).collect::<Vec<String>>();

        request.add_header(header::ACCEPT_ENCODING, &encodings.join(","));

        Ok(None)
    }

    fn on_response(&self, response: &mut DeboaResponse) {
        let response_headers = response.headers();
        let content_encoding = response_headers.get(header::CONTENT_ENCODING);
        if let Some(content_encoding) = content_encoding {
            let decompressor = self.accept_encoding.get(content_encoding.to_str().unwrap());
            if let Some(decompressor) = decompressor {
                let _ = decompressor.decompress_body(response);
            }
        }
    }
}
