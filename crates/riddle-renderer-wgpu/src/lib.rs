#![feature(arc_new_cyclic)]

mod buffered_renderer;
mod error;
mod render_context;
mod renderer;
mod shader;
mod sprite;
mod sprite_atlas;
mod sprite_render_target;
mod swap_chain_target;
mod texture;
mod vertex;

pub mod ext;

pub use buffered_renderer::*;
pub use error::*;
pub use render_context::*;
pub use renderer::*;
pub use sprite::*;
pub use sprite_atlas::*;
pub use sprite_render_target::*;
pub use texture::*;

use riddle_common::*;
use shader::*;
use swap_chain_target::*;
use vertex::*;

use riddle_image as image;
use riddle_math as math;
use riddle_platform_winit as platform;

type Result<R> = std::result::Result<R, RendererError>;
