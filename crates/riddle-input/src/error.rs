use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Unknown Input Error")]
    Unknown,
}

impl From<InputError> for riddle_common::CommonError {
    fn from(e: InputError) -> Self {
        e.into()
    }
}
