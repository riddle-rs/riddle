use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
    #[error("Initialization Error")]
    InitError(&'static str),

    #[error("Unknown Input Error")]
    Unknown,
}
