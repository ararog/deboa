use deboa::{
    errors::{ContentError, DeboaError},
    request::DeboaRequestBuilder,
    response::DeboaResponse,
    Result,
};
use fory::{Fory, ForyDefault, Serializer};
use http::header;

//#[cfg(test)]
//mod tests;

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
            .bytes(&result.unwrap())
            .header(header::CONTENT_TYPE, "application/fory");

        Ok(builder)
    }
}

pub trait ForyResponse {
    fn body_as_fory<T: Serializer + ForyDefault>(
        self,
        fory: &Fory,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
}

impl ForyResponse for DeboaResponse {
    async fn body_as_fory<T: Serializer + ForyDefault>(self, fory: &Fory) -> Result<T> {
        let result = fory.deserialize(&self.bytes().await);
        if let Err(error) = result {
            return Err(DeboaError::Content(ContentError::Deserialization {
                message: error.to_string(),
            }));
        }

        Ok(result.unwrap())
    }
}
