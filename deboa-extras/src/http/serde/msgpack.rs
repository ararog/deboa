use deboa::{
    errors::{ContentError, DeboaError},
    serde::{RequestBody, ResponseBody},
    Result,
};
use mime_typed::{ApplicationMsgpack, MimeStrExt};
use serde::{Deserialize, Serialize};

pub struct MsgPackBody;

impl RequestBody for MsgPackBody {
    fn mime_type(&self) -> &str {
        ApplicationMsgpack::MIME_STR
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let result = rmp_serde::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(result.unwrap())
    }
}

impl ResponseBody for MsgPackBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
        let binding = body;
        let body = binding.as_ref();

        let json = rmp_serde::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                Err(DeboaError::Content(ContentError::Deserialization { message: err.to_string() }))
            }
        }
    }
}
