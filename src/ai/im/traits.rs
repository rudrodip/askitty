use crate::errors::ImageGenError;

pub trait IM {
    fn new(host: String, model: String, api_key: String) -> Result<Self, ImageGenError>
    where
        Self: Sized;
    fn generate(
        &self,
        text: String,
    ) -> impl std::future::Future<Output = Result<(), ImageGenError>> + Send;
}
