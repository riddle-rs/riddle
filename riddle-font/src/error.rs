use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FontError {
	#[error("Image Error")]
	Image(#[from] riddle_image::ImageError),

	#[error("Failed to parse font")]
	FontParseFailed,

	#[error(transparent)]
	Common(#[from] CommonError),
}

impl From<std::io::Error> for FontError {
	fn from(err: std::io::Error) -> Self {
		FontError::Common(CommonError::Io(err))
	}
}
