use deboa::{
    Deboa,
    errors::DeboaError,
    http::serde::{RequestBody, ResponseBody},
};
use http::header;
use mime_typed::Msgpack;
use serde::{Deserialize, Serialize};

pub struct MsgPackBody;

impl RequestBody for MsgPackBody {
    fn register_content_type(&self, deboa: &mut Deboa) {
        deboa.edit_header(header::CONTENT_TYPE, Msgpack.to_string().as_str());
        deboa.edit_header(header::ACCEPT, Msgpack.to_string().as_str());
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        let result = rmp_serde::to_vec(&data);
        if let Err(error) = result {
            return Err(DeboaError::Serialization { message: error.to_string() });
        }

        Ok(result.unwrap())
    }
}

impl ResponseBody for MsgPackBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T, DeboaError> {
        let binding = body;
        let body = binding.as_ref();

        let json = rmp_serde::from_slice(body);

        match json {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Deserialization { message: err.to_string() }),
        }
    }
}
