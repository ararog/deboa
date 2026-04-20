use std::io::Cursor;

use deboa::{
    errors::{ContentError, DeboaError},
    serde::{RequestBody, ResponseBody},
    Result,
};
use mime_typed::{MimeStrExt, TextXml};
use serde::{Deserialize, Serialize};

/// XML body serializer/deserializer
pub struct XmlBody;

impl RequestBody for XmlBody {
    fn mime_type(&self) -> &str {
        TextXml::MIME_STR
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let mut ser_xml_buf = Vec::new();

        let result = serde_xml_rust::to_writer(&mut ser_xml_buf, &data);

        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(ser_xml_buf)
    }
}

impl ResponseBody for XmlBody {
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
