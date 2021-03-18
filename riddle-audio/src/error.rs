use riddle_common::CommonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
	#[error("Error acquiring rodio device")]
	InitFailed { cause: &'static str },

	#[error("Error playing clip")]
	PlayError { cause: &'static str },

	#[error("Error decoding clip")]
	ClipDecodeError,

	#[error(transparent)]
	CommonError(#[from] CommonError),
}

impl From<std::io::Error> for AudioError {
	fn from(err: std::io::Error) -> Self {
		AudioError::CommonError(CommonError::IOError(err))
	}
}
