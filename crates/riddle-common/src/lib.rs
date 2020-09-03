use thiserror::Error;

mod color;
pub mod eventpub;

pub use color::*;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error(transparent)]
    Common(#[from] Box<dyn std::error::Error>),
}
