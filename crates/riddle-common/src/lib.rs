use thiserror::Error;

mod clone_handle;
mod color;

pub mod eventpub;

pub use clone_handle::*;
pub use color::*;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error(transparent)]
    Common(#[from] Box<dyn std::error::Error>),
}
