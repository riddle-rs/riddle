use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error("Unknown Renderer Error")]
    Unknown,
}

impl From<RendererError> for riddle_common::CommonError {
    fn from(e: RendererError) -> Self {
        e.into()
    }
}
