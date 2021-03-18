use crate::{wgpu_ext::*, *};

/////////////////////////////////////////////////////////////////////////////
// struct SpriteBuilder
/////////////////////////////////////////////////////////////////////////////

/// Builder to construct new [`Sprite`]s from `riddle_image::Image`s.
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
/// let sprite = SpriteBuilder::new(img)
///     .with_filter_modes(FilterMode::Linear, FilterMode::Linear)
///     .build(&renderer)?;
/// # Ok(()) }
/// ```
pub struct SpriteBuilder {
	pub(crate) img: image::Image,
	pub(crate) mag_filter: FilterMode,
	pub(crate) min_filter: FilterMode,
}

impl SpriteBuilder {
	/// Create a new builder for the given image
	pub fn new(img: image::Image) -> Self {
		Self {
			img,
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

	/// Build the sprite for the given renderer
	pub fn build<Device: WGPUDevice>(
		self,
		renderer: &WGPURenderer<Device>,
	) -> Result<WGPUSprite<Device>> {
		WGPUSprite::new_from_image(renderer, &self.img, self.mag_filter, self.min_filter)
	}
}
