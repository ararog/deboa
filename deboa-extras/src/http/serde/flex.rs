use deboa::client::serde::{RequestBody, ResponseBody};
use deboa::{
    errors::{ContentError, DeboaError},
    request::DeboaRequest,
    Result,
};
use http::header;
use serde::{Deserialize, Serialize};
pub struct FlexBody;

const FLEXBUFFERS_CONTENT_TYPE: &str = "application/x-flexbuffers";

impl RequestBody for FlexBody {
    fn register_content_type(&self, request: &mut DeboaRequest) {
        request.add_header(header::CONTENT_TYPE, FLEXBUFFERS_CONTENT_TYPE);
        request.add_header(header::ACCEPT, FLEXBUFFERS_CONTENT_TYPE);
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let result = flexbuffers::to_vec(data);

        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(result.unwrap())
    }
}

impl ResponseBody for FlexBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
        let result = flexbuffers::from_slice(&body);

        match result {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                Err(DeboaError::Content(ContentError::Deserialization { message: err.to_string() }))
            }
        }
    }
}
