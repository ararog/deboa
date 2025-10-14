use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use deboa::{
    request::{DeboaRequest, DeboaRequestBuilder},
    url::IntoUrl,
    Result,
};
use http::{header, Method};

pub trait WebsocketRequestBuilder {
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
