#![feature(arc_new_cyclic)]

//! Riddle crate dealing with user input (keyboard, mouse, gamepad).
//!
//! Built largely on the back of `gilrs` and its dependencies for controller support,
//! and consumes system events from a `riddle-platform` compatible system (`winit`).
//!
//! The primary functions of this crate is to convert incoming input event streams in
//! to a queryable immediate-mode-like API, and to unify the incoming event streams in
//! to a single input event stream.
//!
//! # Riddle Example
//!
//! The **recommended** way to use this crate is through the main `riddle` crate.
//! Riddle exposes this crate through `riddle::input`.
//!
//! ```no_run
//! use riddle::{*, platform::*, input::*};
//!
//! fn main() -> Result<(), RiddleError> {
//!     let rdl =  RiddleLib::new()?;
//!     let window = WindowBuilder::new().build(rdl.context())?;
//!
//!     rdl.run(move |rdl| {
//!         match rdl.event() {
//!             Event::Input(InputEvent::KeyDown { .. }) => rdl.quit(),
//!             _ => (),
//!         }
//!     })
//! }
//! ```
//!
//! # Direct Example
//!
//! If you don't want to depend on `riddle`, you can use this crate directly. In order to have
//! the crate function, appropriate `riddle_platform_common::PlatformEvent`s must be sent
//! to the input system via the `EventPub` it is constructed with.
//!
//! ```no_run
//! use riddle_input::{ext::*, *};
//! use riddle_common::eventpub::*;
//! use riddle_platform_common::*;
//!
//! fn main() -> Result<(), InputError> {
//!     let platform_events: EventPub<PlatformEvent> = EventPub::new();
//!     let (input_system, mut main_thread_state) = InputSystem::new(&platform_events)?;
//!
//!     platform_events.dispatch(PlatformEvent::MouseButtonDown{
//!         window: WindowId::new(0),
//!         button: MouseButton::Left});
//!
//!     main_thread_state.process_input();
//!
//!     let input_events: Vec<InputEvent> = input_system.take_input_events();
//!
//!     Ok(())
//! }
//! ```

mod error;
mod event;
mod gamepad;
mod gamepad_state;
mod input_system;
mod keyboard_state;
mod mouse_state;

pub mod ext;

use gamepad_state::*;
use mouse_state::*;

pub use error::*;
pub use event::*;
pub use gamepad::*;
pub use input_system::*;
pub use keyboard_state::*;
pub use riddle_platform_common::{LogicalPosition, MouseButton, Scancode, VirtualKey};

type Result<R> = std::result::Result<R, InputError>;
