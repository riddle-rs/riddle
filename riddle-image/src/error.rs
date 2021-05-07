use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
	#[error("Image decoding error: {0}")]
	Load(&'static str),

	#[error("Image encoding error: {0}")]
	Save(&'static str),

	#[error("Image packing error: {0}")]
	Packing(&'static str),

	#[error(transparent)]
	Common(#[from] CommonError),
}

impl From<ImageError> for CommonError {
	fn from(e: ImageError) -> Self {
		e.into()
	}
}

impl From<std::io::Error> for ImageError {
	fn from(err: std::io::Error) -> Self {
		ImageError::Common(CommonError::Io(err))
	}
}
