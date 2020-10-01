//! Riddle crate supplying basic math types and utilities to allow easy interop between
//! riddle crates.
//!
//! The crate also re-exports `mint`, which is used in some of the public APIs. Mint has
//! become somewhat of a standard for linear algebra type interop.
//!
//! Other riddle crates, and users of riddle should not rely on this crate for efficient
//! math operations - it provides API level utilities. Libraries such as `cgmath`, `glam`,
//! and `nalgebra`, to name but a few options, should be used by client code to perform
//! efficient linear algebra calculations where needed.

mod rect;
mod spacial_numeric;
mod vector;

pub use rect::*;
pub use spacial_numeric::*;
pub use vector::*;

pub use mint;
