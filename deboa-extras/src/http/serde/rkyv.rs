use std::io::Cursor;

use deboa::client::serde::{RequestBody, ResponseBody};
use deboa::{
    errors::{ContentError, DeboaError},
    request::DeboaRequest,
    Result,
};
use http::header;
use mime_typed::Xml;
use rkyv::{deserialize, rancor::Error, Archive, Deserialize, Serialize};


pub struct RkyvBody;

impl RequestBody for RkyvBody {
    fn register_content_type(&self, request: &mut DeboaRequest) {
        request.add_header(
            header::CONTENT_TYPE,
            Xml.to_string()
                .as_str(),
        );
        request.add_header(
            header::ACCEPT,
            Xml.to_string()
                .as_str(),
        );
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let result = rkyv::to_bytes::<Error>(&data);

        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(ser_xml_buf)
    }
}

impl ResponseBody for RkyvBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
        let xml = serde_xml_rust::from_reader(Cursor::new(body));

        match xml {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                Err(DeboaError::Content(ContentError::Deserialization { message: err.to_string() }))
            }
        }
    }
}
