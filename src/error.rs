use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RiddleError {
    #[error(transparent)]
    AudioError(#[from] audio::AudioError),
    #[error(transparent)]
    FontError(#[from] font::FontError),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    InputError(#[from] input::InputError),
    #[error(transparent)]
    RendererError(#[from] renderer::RendererError),
    #[error(transparent)]
    WindowError(#[from] window::WindowError),
}

impl From<RiddleError> for riddle_common::CommonError {
    fn from(e: RiddleError) -> Self {
        e.into()
    }
}
