use deboa::{
    request::{DeboaRequest, DeboaRequestBuilder},
    url::IntoUrl,
    Result,
};
use http::{header, Method};
use mime_typed::{MimeStrExt, TextEventStream};

pub trait ServerSentEventBuilder {
    fn sse<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder>;
}

impl ServerSentEventBuilder for DeboaRequestBuilder {
    fn sse<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::at(url, Method::GET)?
            .header(header::CONTENT_TYPE, TextEventStream::MIME_STR))
    }
}
