#![feature(arc_new_cyclic)]

/*!
Riddle crate with miscelanious support functionality required by
the rest of the riddle crates.
*/

mod clone_handle;
mod color;
mod error;

pub mod eventpub;

pub use clone_handle::*;
pub use color::*;
pub use error::*;
