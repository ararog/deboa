use deboa::{fs::io::Decompressor, interceptor::DeboaInterceptor, request::DeboaRequest, response::DeboaResponse};
use http::header;
use std::collections::HashMap;

pub struct EncodingInterceptor {
    pub accept_encoding: HashMap<String, Box<dyn Decompressor>>,
}

impl EncodingInterceptor {
    pub fn register_decoders(decoders: Vec<Box<dyn Decompressor>>) -> Self {
        let mut accept_encoding = HashMap::new();
        for decoder in decoders {
            accept_encoding.insert(decoder.name(), decoder);
        }
        Self { accept_encoding }
    }
}

impl DeboaInterceptor for EncodingInterceptor {
    fn on_request(&self, request: &mut DeboaRequest) {
        let encodings = self.accept_encoding.values().map(|decoder| decoder.name()).collect::<Vec<String>>();

        request.add_header(header::ACCEPT_ENCODING, &encodings.join(","));
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
