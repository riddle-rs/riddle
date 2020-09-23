#![feature(arc_new_cyclic)]

mod dimensions;
mod error;
mod event;
mod platform_context;
mod platform_system;
mod window;
mod window_map;

pub use error::PlatformError;
pub use platform_context::*;
pub use platform_system::*;
pub use riddle_platform_common::*;
pub use window::*;

use riddle_common::*;
use window_map::*;

type Result<R> = std::result::Result<R, PlatformError>;
