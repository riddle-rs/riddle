#![feature(arc_new_cyclic)]

mod error;
mod event;
mod gamepad;
mod gamepad_state;
mod input_system;
mod keyboard_state;
mod mouse_state;

use gamepad_state::*;

pub use error::*;
pub use event::*;
pub use gamepad::*;
pub use input_system::*;
pub use keyboard_state::*;
pub use mouse_state::*;
pub use riddle_platform_common::{LogicalPosition, MouseButton, Scancode, VirtualKey};
