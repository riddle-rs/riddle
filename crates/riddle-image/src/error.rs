use riddle_common::CommonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Unknown Image Error")]
    Unknown,
}

impl From<ImageError> for CommonError {
    fn from(e: ImageError) -> Self {
        e.into()
    }
}
