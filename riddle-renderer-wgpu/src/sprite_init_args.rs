use crate::wgpu_ext::*;

/////////////////////////////////////////////////////////////////////////////
// struct SpriteBuilder
/////////////////////////////////////////////////////////////////////////////

/// Represents the settings which are used when constructing a sprite.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// let img = Image::new(100, 100);
/// let sprite = Sprite::new_from_image(&renderer, &img, &SpriteInitArgs::new()
///     .with_filter_modes(FilterMode::Linear, FilterMode::Linear))?;
/// # Ok(()) }
/// ```
pub struct SpriteInitArgs {
	pub mag_filter: FilterMode,
	pub min_filter: FilterMode,
}

impl SpriteInitArgs {
	/// Create a new default init args.
	pub fn new() -> Self {
		Self {
			mag_filter: Default::default(),
			min_filter: Default::default(),
		}
	}

	/// Specify the min and mag filters used when rendering the sprite
	pub fn with_filter_modes(mut self, mag_filter: FilterMode, min_filter: FilterMode) -> Self {
		self.mag_filter = mag_filter;
		self.min_filter = min_filter;
		self
	}
}
