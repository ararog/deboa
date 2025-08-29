use http::{HeaderMap, StatusCode};

pub trait DeboaResponse {
    fn status(&self) -> StatusCode {
        unimplemented!()
    }

    fn headers(&self) -> HeaderMap {
        unimplemented!()
    }

    fn raw_body(&self) -> Vec<u8> {
        unimplemented!()
    }
}
