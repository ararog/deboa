use deboa::{
    errors::{ContentError, DeboaError},
    request::DeboaRequest,
    serde::{RequestBody, ResponseBody},
    Result,
};
use http::{header, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
pub struct YamlBody;

const YAML_CONTENT_TYPE: &str = "application/yaml";

impl RequestBody for YamlBody {
    fn mime_type(&self) -> &str {
        YAML_CONTENT_TYPE
    }

    fn serialize<T: Serialize>(&self, data: T) -> Result<Vec<u8>> {
        let mut ser_yaml_buf = Vec::new();

        let result = serde_saphyr::to_io_writer(&mut ser_yaml_buf, &data);

        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        Ok(ser_yaml_buf)
    }
}

impl ResponseBody for YamlBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, body: Vec<u8>) -> Result<T> {
        let yaml = serde_saphyr::from_slice(&body);

        match yaml {
            Ok(deserialized_body) => Ok(deserialized_body),
            Err(err) => {
                Err(DeboaError::Content(ContentError::Deserialization { message: err.to_string() }))
            }
        }
    }
}
