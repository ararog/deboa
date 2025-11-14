use deboa::{
    errors::{ContentError, DeboaError},
    request::DeboaRequestBuilder,
    response::DeboaResponse,
    Result,
};
use fory::{Fory, ForyDefault, Serializer};
use http::header;

#[cfg(test)]
mod tests;

pub trait ForyRequestBuilder {
    fn body_as_fory<T: Serializer>(self, fory: &Fory, body: T) -> Result<DeboaRequestBuilder>;
}

impl ForyRequestBuilder for DeboaRequestBuilder {
    fn body_as_fory<T: Serializer>(self, fory: &Fory, body: T) -> Result<DeboaRequestBuilder> {
        let result = fory.serialize(&body);
        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Serialization {
                message: error.to_string(),
            }));
        }

        let builder = self
            .raw_body(&result.unwrap())
            .header(header::CONTENT_TYPE, "application/fory");

        Ok(builder)
    }
}

#[deboa::async_trait]
pub trait ForyResponse {
    async fn body_as_fory<T: Serializer + ForyDefault>(&mut self, fory: &Fory) -> Result<T>;
}

#[deboa::async_trait]
impl ForyResponse for DeboaResponse {
    async fn body_as_fory<T: Serializer + ForyDefault>(&mut self, fory: &Fory) -> Result<T> {
        let result = fory.deserialize(
            &self
                .raw_body()
                .await,
        );
        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Deserialization {
                message: error.to_string(),
            }));
        }

        Ok(result.unwrap())
    }
}
