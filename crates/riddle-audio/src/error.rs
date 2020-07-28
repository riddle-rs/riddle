use riddle_common::CommonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Initialization Error")]
    UnknownError,
}

impl From<AudioError> for CommonError {
    fn from(e: AudioError) -> Self {
        e.into()
    }
}
