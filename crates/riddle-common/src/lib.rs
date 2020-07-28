use thiserror::Error;

pub mod eventpub;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error(transparent)]
    Common(#[from] Box<dyn std::error::Error>),
}
