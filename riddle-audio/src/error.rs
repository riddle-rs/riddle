use riddle_common::CommonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Error acquiring rodio device")]
    InitFailed { cause: &'static str },

    #[error("Error decoding clip")]
    ClipDecodeError,

    #[error(transparent)]
    CommonError(#[from] CommonError),
}
