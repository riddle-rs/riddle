use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
	#[error("Unknown Image Error")]
	Unknown,

	#[error(transparent)]
	CommonError(#[from] CommonError),
}

impl From<ImageError> for CommonError {
	fn from(e: ImageError) -> Self {
		e.into()
	}
}

impl From<::image::ImageError> for ImageError {
	fn from(err: ::image::ImageError) -> Self {
		match err {
			::image::ImageError::Decoding(_) => ImageError::Unknown,
			::image::ImageError::Encoding(_) => ImageError::Unknown,
			::image::ImageError::Parameter(_) => ImageError::Unknown,
			::image::ImageError::Limits(_) => ImageError::Unknown,
			::image::ImageError::Unsupported(_) => ImageError::Unknown,
			::image::ImageError::IoError(e) => CommonError::IOError(e).into(),
		}
	}
}

impl From<std::io::Error> for ImageError {
	fn from(err: std::io::Error) -> Self {
		ImageError::CommonError(CommonError::IOError(err))
	}
}
