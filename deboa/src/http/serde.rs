use crate::errors::DeboaError;
use serde::{Deserialize, Serialize};

pub trait RequestBody {
    fn serialize<T: Serialize>(&self, value: T) -> Result<Vec<u8>, DeboaError>;
}

pub trait ResponseBody {
    fn deserialize<T: for<'a> Deserialize<'a>>(&self, value: Vec<u8>) -> Result<T, DeboaError>;
}
