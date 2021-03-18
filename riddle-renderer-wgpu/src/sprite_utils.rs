use crate::{math::*, wgpu_ext::*, *};

/////////////////////////////////////////////////////////////////////////////
// struct SpriteRenderArgs
/////////////////////////////////////////////////////////////////////////////

/// Builder for a [`Sprite`] render call
///
/// Defaults:
///
/// * **Pivot**: `(0,0)`
/// * **Scale**: `(1,1)`
/// * **Angle**: `0`
/// * **Diffuse Color**: `Color::WHITE`
///
/// The location refers to the location of the pivot of the sprite. The pivot
/// of the sprite is relative to the top left of the sprite.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// # let renderer = Renderer::new_from_window(&window)?;
/// # let img = Image::new(100, 100);
/// let sprite = SpriteBuilder::new(img).build(&renderer)?;
///
/// let mut render_ctx = renderer.begin_render()?;
/// render_ctx.clear(Color::WHITE)?;
///
/// // Render the sprite
/// SpriteRenderArgs::new(vec2(0.0, 0.0))
///     .with_scale(vec2(1.0, 2.0))
///     .with_color(Color::RED)
///     .render(&mut render_ctx, &sprite)?;
///
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct SpriteRenderArgs {
	pub location: Vector2<f32>,
	pub pivot: Vector2<f32>,
	pub scale: Vector2<f32>,
	pub angle: f32,
	pub diffuse_color: Color<f32>,
}

impl SpriteRenderArgs {
	/// New render command with default args, at the specified location
	pub fn new<T: Into<Vector2<f32>>>(location: T) -> Self {
		let mut args = Self::default();
		args.at(location);
		args
	}

	/// Set the location of the sprite, specifying where the pivot should
	/// be placed.
	#[inline]
	pub fn at<T: Into<Vector2<f32>>>(&mut self, location: T) -> &mut Self {
		self.location = location.into();
		self
	}

	/// Set the pivot of the sprite, relative to the top left of the sprite
	#[inline]
	pub fn with_pivot<T: Into<Vector2<f32>>>(&mut self, pivot: T) -> &mut Self {
		self.pivot = pivot.into();
		self
	}

	/// Set the scale at which the sprite will be rendered
	pub fn with_scale<T: Into<Vector2<f32>>>(&mut self, scale: T) -> &mut Self {
		self.scale = scale.into();
		self
	}

	/// Set the angle at which the sprite will be rendered, in radians.
	pub fn with_angle(&mut self, angle: f32) -> &mut Self {
		self.angle = angle;
		self
	}

	/// Set the diffuse color of the sprite, which will be multiplied by the sprite
	/// colors.
	pub fn with_color(&mut self, color: Color<f32>) -> &mut Self {
		self.diffuse_color = color;
		self
	}

	/// Invoke the render command, for the given sprite, in the specified context
	pub fn render<Device>(
		&self,
		render_ctx: &mut impl RenderContext,
		sprite: &WGPUSprite<Device>,
	) -> Result<()>
	where
		Device: WGPUDevice,
	{
		sprite.render(render_ctx, self)
	}
}

impl Default for SpriteRenderArgs {
	fn default() -> Self {
		SpriteRenderArgs {
			location: [0.0, 0.0].into(),
			pivot: [0.0, 0.0].into(),
			angle: 0.0,
			scale: [1.0, 1.0].into(),
			diffuse_color: Color::WHITE,
		}
	}
}

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
