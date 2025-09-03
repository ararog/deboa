use std::collections::HashMap;

use crate::{errors::DeboaError, io::Decompressor, response::DeboaResponse};
use bytes::{Buf, Bytes};
use http::{header, HeaderName, HeaderValue, Request};
use http_body_util::{BodyExt, Full};
use hyper::client::conn::http1::SendRequest;
use url::Url;

pub struct DeboaConnection {
    url: Url,
    sender: SendRequest<Full<Bytes>>,
}

impl DeboaConnection {
    pub fn new(url: Url, sender: SendRequest<Full<Bytes>>) -> Self {
        Self { url, sender }
    }

    pub fn set_url(&mut self, url: Url) {
        self.url = url;
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub async fn send_request(
        &mut self,
        method: http::Method,
        headers: Option<&HashMap<HeaderName, String>>,
        encodings: Option<&HashMap<String, Box<dyn Decompressor>>>,
        body: Vec<u8>,
    ) -> Result<DeboaResponse, DeboaError> {
        let authority = self.url.authority();

        let mut builder = Request::builder()
            .uri(self.url.as_str())
            .method(method.to_string().as_str())
            .header(hyper::header::HOST, authority);
        {
            let req_headers = builder.headers_mut().unwrap();
            if let Some(headers) = &headers {
                headers.iter().fold(req_headers, |acc, (key, value)| {
                    acc.insert(key, HeaderValue::from_str(value).unwrap());
                    acc
                });
            }
        }

        let request = builder.body(Full::new(Bytes::from(body.to_vec())));
        if let Err(err) = request {
            return Err(DeboaError::Request {
                host: self.url.host().unwrap().to_string(),
                path: self.url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let response = self.sender.send_request(request.unwrap()).await;
        if let Err(err) = response {
            return Err(DeboaError::Request {
                host: self.url.host().unwrap().to_string(),
                path: self.url.path().to_string(),
                method: method.to_string(),
                message: err.to_string(),
            });
        }

        let response = response.unwrap();

        let status_code = response.status();
        let headers = response.headers().clone();

        let result = response.collect().await;

        if let Err(err) = result {
            return Err(DeboaError::Response {
                status_code: status_code.as_u16(),
                message: err.to_string(),
            });
        }

        let mut response_body = result.unwrap().aggregate();

        let raw_body = response_body.copy_to_bytes(response_body.remaining()).to_vec();

        if !status_code.is_success() {
            return Err(DeboaError::Response {
                status_code: status_code.as_u16(),
                message: format!("Request failed with status code: {status_code}"),
            });
        }

        #[cfg(feature = "middlewares")]
        let mut response = DeboaResponse::new(status_code, headers, &raw_body);

        if let Some(encodings) = &encodings {
            let response_headers = response.headers();
            let content_encoding = response_headers.get(header::CONTENT_ENCODING);
            if let Some(content_encoding) = content_encoding {
                let decompressor = encodings.get(content_encoding.to_str().unwrap());
                if let Some(decompressor) = decompressor {
                    decompressor.decompress_body(&mut response)?;
                }
            }
        }

        Ok(response)
    }
}

#[async_trait::async_trait]
pub trait HttpConnection: Send + Sync + 'static {
    async fn connect(url: Url) -> Result<DeboaConnection, DeboaError>;
}
