use std::io::Cursor;

use deboa::{
    errors::{ContentError, DeboaError},
    serde::{RequestBody, ResponseBody},
    Result,
};
use serde::{Deserialize, Serialize};

/// CBOR body serializer/deserializer
pub struct CborBody;

const APPLICATION_CBOR: &str = "application/cbor";

impl RequestBody for CborBody {
    fn mime_type(&self) -> &str {
        APPLICATION_CBOR
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        let result = ciborium::ser::into_writer(&data, &mut buf);

        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(buf)
    }
}

impl ResponseBody for CborBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
        let result = ciborium::de::from_reader(Cursor::new(body));

        match result {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                let content_err = ContentError::Deserialization { message: err.to_string() };
                Err(DeboaError::Content(content_err))
            }
        }
    }
}
