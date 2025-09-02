use std::io::Cursor;

use deboa::errors::DeboaError;
use deboa::http::serde::{RequestBody, ResponseBody};
use mime_typed::xml::XML;
use serde::{Deserialize, Serialize};
pub struct XmlBody;

impl RequestBody for XmlBody {
    fn register_content_type(&self, deboa: &mut Deboa) {
        deboa.edit_header(header::CONTENT_TYPE, XML.to_string());
        deboa.edit_header(header::ACCEPT, XML.to_string());
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>, DeboaError> {
        let mut ser_xml_buf = Vec::new();

        let result = serde_xml_rust::to_writer(&mut ser_xml_buf, &data);

        if let Err(error) = result {
            return Err(DeboaError::Serialization { message: error.to_string() });
        }

        Ok(ser_xml_buf)
    }
}

impl ResponseBody for XmlBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T, DeboaError> {
        let xml = serde_xml_rust::from_reader(Cursor::new(body));

        match xml {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => Err(DeboaError::Deserialization { message: err.to_string() }),
        }
    }
}
