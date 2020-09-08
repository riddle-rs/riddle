use thiserror::Error;

#[derive(Debug, Error)]
pub enum WindowError {
    #[error("Unknown Window Error")]
    Unknown,
}

impl From<WindowError> for riddle_common::CommonError {
    fn from(e: WindowError) -> Self {
        e.into()
    }
}
