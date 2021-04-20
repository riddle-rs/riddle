#![feature(arc_new_cyclic)]
#![deny(clippy::all)]

//! Riddle simple sprite-based renderer built on `wgpu`.
//!
//! The renderer can be used by itself as the only renderer in your application,
//! or it can be constructed on top of existing WGPU devices to provide simple
//! 2D rendering.
//!
//! # Riddle Example
//!
//! The **recommended** way to use this crate is through the main `riddle` crate.
//! Riddle exposes this crate through `riddle::renderer`. The following example
//! lets this crate take full ownership over creating the WGPU device.
//!
//! ```no_run
//! use riddle::{common::Color, platform::*, renderer::*, *};
//!
//! fn main() -> Result<(), RiddleError> {
//!     let rdl =  RiddleLib::new()?;
//!     let window = WindowBuilder::new().build(rdl.context())?;
//!     let renderer = Renderer::new_from_window(&window)?;
//!
//!     rdl.run(move |rdl| {
//!         match rdl.event() {
//!             Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
//!             Event::ProcessFrame => {
//!                 renderer.render(|render_ctx| {
//!                     render_ctx.clear(Color::RED)
//!                 }).unwrap();
//!             },
//!             _ => ()
//!          }
//!     })
//! }
//! ```
//!
//! # Direct Usage
//!
//! To use this crate directly see [`WGPUDevice`] for how to use the renderer
//! with custom WGPU devices.

mod buffered_renderer;
mod device;
mod error;
mod renderer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_render_target;
mod swap_chain_target;
mod target;
mod texture;
mod window_device;

use riddle_common::*;
use riddle_image as image;
use riddle_math as math;
use riddle_platform_winit as platform;

type Result<R> = std::result::Result<R, WGPURendererError>;

use buffered_renderer::*;
pub use device::*;
pub use error::*;
pub use renderer::*;
use shader::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_render_target::*;
use swap_chain_target::*;
use target::*;
use texture::*;
pub use window_device::*;

pub use riddle_renderer_common::*;

use riddle_renderer_common::vertex::*;

pub type DefaultRenderer = Renderer<WindowWGPUDevice>;
