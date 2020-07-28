/*!
A crate for loading font files and rendering text to images.


*/

use thiserror::Error;

mod ttfont;

pub use ttfont::TTFont;

#[derive(Debug, Error)]
pub enum FontError {
    #[error("Image Error")]
    ImageError(riddle_image::ImageError),

    #[error("Unknown Error")]
    UnknownError,
}

impl From<FontError> for riddle_common::CommonError {
    fn from(e: FontError) -> Self {
        e.into()
    }
}

impl From<riddle_image::ImageError> for FontError {
    fn from(e: riddle_image::ImageError) -> Self {
        FontError::ImageError(e)
    }
}
