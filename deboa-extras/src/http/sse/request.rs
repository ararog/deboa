use deboa::{
    request::{DeboaRequest, DeboaRequestBuilder},
    url::IntoUrl,
    Result,
};
use http::{header, Method};
use mime_typed::{MimeStrExt, TextEventStream};

/// Trait to convert a DeboaRequestBuilder into a SSE request.
pub trait ServerSentEventBuilder {
    /// Converts a DeboaRequestBuilder into a SSE request.
    ///
    /// # Returns
    ///
    /// A DeboaRequestBuilder.
    ///
    /// # Examples
    ///
    /// ``` compile_fail
    /// use deboa::{Deboa, Result, request::{IntoUrl, DeboaRequestBuilder}};
    /// use deboa_extras::http::sse::request::{ServerSentEventBuilder};
    ///
    /// let mut client = Deboa::new();
    /// let builder = DeboaRequestBuilder::sse("https://sse.dev/test").unwrap();
    /// let response = builder.go(&mut client).await.unwrap();
    /// let event_stream = response.into_event_stream().unwrap();
    /// while let Some(event) = event_stream.next().await {
    ///     println!("event: {}", event);
    /// }
    /// ```
    fn sse<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder>;
}

impl ServerSentEventBuilder for DeboaRequestBuilder {
    fn sse<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        Ok(DeboaRequest::at(url, Method::GET)?
            .header(header::CONTENT_TYPE, TextEventStream::MIME_STR))
    }
}
