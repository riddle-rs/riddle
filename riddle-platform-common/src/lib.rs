//! Riddle crate containing common utilities and types needed by platform implementations
//! and which other crates can use to interact with a platform service without needing to
//! know or genericize for the platform system type being used.
//!
//! Most types in here are either consumed or reexported through the concrete platform
//! crate (`riddle_platform_winit`). There should be very few cases where this crate will
//! need to be depended on directly.

mod dimensions;
mod event;
mod mouse;
mod scancode;
mod virtualkey;
mod window_id;

pub mod doctest;
pub mod traits;

pub use dimensions::*;
pub use event::*;
pub use mouse::*;
pub use scancode::*;
pub use virtualkey::*;
pub use window_id::*;
