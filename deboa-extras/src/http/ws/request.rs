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
        let key = rand::random::<u128>().to_string();
        Ok(DeboaRequest::at(url, Method::GET)?
            .header(header::UPGRADE, "websocket")
            .header(header::CONNECTION, "Upgrade")
            .header(header::SEC_WEBSOCKET_KEY, &key)
            .header(header::SEC_WEBSOCKET_VERSION, "13"))
    }
}
