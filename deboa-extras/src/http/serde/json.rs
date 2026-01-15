use deboa::{
    client::serde::{RequestBody, ResponseBody},
    errors::{ContentError, DeboaError},
    request::DeboaRequest,
};
use http::header;
use mime_typed::Json;
use serde::{Deserialize, Serialize};
#[cfg(feature = "simd_json")]
use std::io::Cursor;

pub struct JsonBody;

impl RequestBody for JsonBody {
    fn register_content_type(&self, request: &mut DeboaRequest) {
        request.add_header(
            header::CONTENT_TYPE,
            Json.to_string()
                .as_str(),
        );
        request.add_header(
            header::ACCEPT,
            Json.to_string()
                .as_str(),
        );
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        #[cfg(feature = "sonic_json")]
        let result = sonic_rs::to_vec(&data);
        #[cfg(feature = "simd_json")]
        let result = simd_json::to_vec(&data);

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
        #[cfg(feature = "sonic_json")]
        let json = {
            let binding = body;
            let body = binding.as_ref();
            sonic_rs::from_slice(body)
        };

        #[cfg(feature = "simd_json")]
        let json = { simd_json::from_reader(Cursor::new(body)) };

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                Err(DeboaError::Content(ContentError::Deserialization { message: err.to_string() }))
            }
        }
    }
}
