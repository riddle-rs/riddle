use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FontError {
    #[error("Image Error")]
    ImageError(riddle_image::ImageError),

    #[error("Failed to parse font")]
    FontParseFailed,

    #[error(transparent)]
    CommonError(#[from] CommonError),
}

impl From<riddle_image::ImageError> for FontError {
    fn from(e: riddle_image::ImageError) -> Self {
        FontError::ImageError(e)
    }
}
