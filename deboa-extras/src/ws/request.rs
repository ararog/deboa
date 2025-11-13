use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use deboa::{
    request::{DeboaRequest, DeboaRequestBuilder},
    url::IntoUrl,
    Result,
};
use http::{header, Method};

/// Trait for building websocket requests
pub trait WebsocketRequestBuilder {
    /// Creates a websocket request
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to connect to
    ///
    /// # Returns
    ///
    /// A Result containing the DeboaRequestBuilder
    ///
    /// # Example
    ///
    /// ``` compile_fail
    /// use deboa::{Deboa, Result, request::{IntoUrl, DeboaRequestBuilder}};
    /// use deboa_extras::http::ws::request::{WebsocketRequestBuilder};
    ///
    /// let mut client = Deboa::new();
    /// let request = DeboaRequestBuilder::websocket("ws://example.com").unwrap();
    /// let response = request.send_with(&mut client).await.unwrap();
    /// let ws = response.into_websocket().unwrap();
    /// loop {
    ///     if let Ok(Some(message)) = ws.read_message().await {
    ///         println!("message: {}", message);
    ///     }
    /// }
    /// ```
    fn websocket<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder>;
}

impl WebsocketRequestBuilder for DeboaRequestBuilder {
    fn websocket<T: IntoUrl>(url: T) -> Result<DeboaRequestBuilder> {
        let rnd: [u8; 16] = rand::random();
        let key = STANDARD.encode(rnd);
        Ok(DeboaRequest::at(url, Method::GET)?
            .header(header::UPGRADE, "websocket")
            .header(header::CONNECTION, "Upgrade")
            .header(header::SEC_WEBSOCKET_KEY, &key)
            .header(header::SEC_WEBSOCKET_VERSION, "13"))
    }
}
