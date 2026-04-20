use deboa::{
    errors::{ContentError, DeboaError},
    serde::{RequestBody, ResponseBody},
    Result,
};
use mime_typed::MimeStrExt;
use serde::{Deserialize, Serialize};
#[cfg(feature = "simd_json")]
use std::io::Cursor;

pub struct JsonBody;

impl RequestBody for JsonBody {
    fn mime_type(&self) -> &str {
        mime_typed::ApplicationJson::MIME_STR
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
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
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
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
