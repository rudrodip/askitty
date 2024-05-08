use crate::errors::ImageGenError;

pub trait IM {
    fn new() -> Result<Self, ImageGenError>
    where
        Self: Sized;
    fn generate(
        &self,
        text: String,
    ) -> impl std::future::Future<Output = Result<(), ImageGenError>> + Send;
}
