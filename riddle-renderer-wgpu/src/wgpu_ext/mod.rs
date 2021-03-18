//! Traits and structs required to use custom WGPU devices.
//!
//! If using [`Renderer`] by itself through `riddle` there should be no need to use this
//! module.

use crate::{
	eventpub::EventSub,
	math::*,
	platform::{PlatformEvent, Window, WindowHandle},
	*,
};

use std::sync::Mutex;

mod buffered_renderer;
mod device;
mod render_context;
mod renderable;
mod renderer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_font;
mod sprite_render_target;
mod swap_chain_target;
mod target;
mod texture;
mod vertex;
mod window_device;

pub use device::*;
pub use render_context::*;
pub use renderer::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_font::*;
pub use sprite_render_target::*;
pub use texture::*;
pub use window_device::*;

use buffered_renderer::*;
use renderable::*;
use shader::*;
use swap_chain_target::*;
use target::*;
use vertex::*;
