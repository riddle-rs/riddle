#![deny(clippy::all)]

//! Riddle crate with miscelanious support functionality required by
//! the rest of the riddle crates.

mod color;
mod error;

pub mod eventpub;

pub use color::*;
pub use error::*;
