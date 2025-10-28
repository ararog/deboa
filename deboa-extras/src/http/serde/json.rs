use deboa::{
    client::serde::{RequestBody, ResponseBody}, errors::{ContentError, DeboaError}, request::DeboaRequest
};
use http::header;
use mime_typed::Json;
use serde::{Deserialize, Serialize};

pub struct JsonBody;

impl RequestBody for JsonBody {
    fn register_content_type(&self, request: &mut DeboaRequest) {
        request.add_header(header::CONTENT_TYPE, Json.to_string().as_str());
        request.add_header(header::ACCEPT, Json.to_string().as_str());
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        let result = serde_json::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(result.unwrap())
    }
}

impl ResponseBody for JsonBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T, DeboaError> {
        let binding = body;
        let body = binding.as_ref();

        let json = serde_json::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Content(ContentError::Deserialization {
                message: err.to_string(),
            })),
        }
    }
}
