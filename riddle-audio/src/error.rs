use riddle_common::CommonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
	#[error("Error acquiring rodio device: {0}")]
	InitFailed(&'static str),

	#[error("Error playing clip: {0}")]
	Playback(&'static str),

	#[error("Error decoding clip")]
	ClipDecode,

	#[error(transparent)]
	Common(#[from] CommonError),
}

impl From<std::io::Error> for AudioError {
	fn from(err: std::io::Error) -> Self {
		AudioError::Common(CommonError::Io(err))
	}
}
