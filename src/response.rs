use anyhow::Result;
use bytes::Buf;
use http::{HeaderMap, StatusCode};
#[cfg(feature = "json")]
use serde::Deserialize;

pub struct DeboaResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Box<dyn Buf>,
}

impl DeboaResponse {
    #[cfg(feature = "json")]
    pub async fn json<T: for<'a> Deserialize<'a>>(&mut self) -> Result<T> {
        let body = self.body.as_mut();
        let json = serde_json::from_reader(body.reader());
        if let Err(err) = json {
            return Err(err.into());
        }

        Ok(json.unwrap())
    }

    pub async fn text(&mut self) -> Result<String> {
        let body = self.body.as_mut();
        let text = body.copy_to_bytes(body.remaining()).to_vec();
        Ok(String::from_utf8(text)?)
    }
}
