//! Riddle crate containing providing the common API to which riddle renderers abide. This allows
//! secondary libraries to be defined in terms of the traits and structs defined in this crate
//! without needing to encode knowledge of any specific renderers.

mod renderer;
mod sprite;
mod sprite_font;
pub mod vertex;

pub use renderer::*;
pub use sprite::*;
pub use sprite_font::*;

use riddle_common::Color;
use riddle_image::Image;
use riddle_math::{Rect, Vector2};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RendererError {
	#[error("Unkown")]
	Unknown,
}

type Result<T> = std::result::Result<T, RendererError>;
