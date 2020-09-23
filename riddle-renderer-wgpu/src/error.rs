use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
    #[error("Error initializing graphics API")]
    APIInitError(&'static str),

    #[error("Error beginning render")]
    BeginRenderError(&'static str),

    #[error(transparent)]
    ImageError(#[from] image::ImageError),

    #[error(transparent)]
    CommonError(#[from] CommonError),

    #[error("Unknown Renderer Error")]
    Unknown,
}
