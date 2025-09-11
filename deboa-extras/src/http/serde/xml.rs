use std::io::Cursor;

use deboa::client::serde::{RequestBody, ResponseBody};
use deboa::{errors::DeboaError, request::DeboaRequest};
use http::header;
use mime_typed::Xml;
use serde::{Deserialize, Serialize};
pub struct XmlBody;

impl RequestBody for XmlBody {
    fn register_content_type(&self, request: &mut DeboaRequest) {
        request.add_header(header::CONTENT_TYPE, Xml.to_string().as_str());
        request.add_header(header::ACCEPT, Xml.to_string().as_str());
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
