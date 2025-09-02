use crate::{errors::DeboaError, Deboa};
use serde::{Deserialize, Serialize};

pub trait RequestBody {
    fn register_content_type(&self, deboa: &mut Deboa) -> ();
    fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>, DeboaError>;
}

pub trait ResponseBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, value: Vec<u8>) -> Result<T, DeboaError>;
}
