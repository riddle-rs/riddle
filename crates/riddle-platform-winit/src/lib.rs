#![feature(arc_new_cyclic)]

mod dimensions;
mod error;
mod event;
mod platform_system;
mod window;
mod window_map;

pub use error::WindowError;
pub use platform_system::*;
pub use riddle_platform_common::*;
pub use window::{Window, WindowBuilder};

use window_map::*;
