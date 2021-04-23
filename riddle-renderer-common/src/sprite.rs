use crate::*;

/// Sprites are conceptually both a reference to an image, and the sub region of the image
/// which represents the logical sprite.
pub trait CommonSprite<R: CommonRenderer>: Sized {
	/// Construct a new sprite from an image. The image contents are copied to a texture
	/// in RGBA8 format. The entire image will be used
	fn new_from_image(renderer: &R, img: &Image, init_args: &SpriteInitArgs) -> Result<Self>;

	/// Build a sprite that shares the same underlying texture but represents a different portion
	/// of the texture.
	///
	/// # Arguments
	///
	/// * **source_rect** - The portion of the texture that the new sprite will render, relative to
	///                     the current sprite's bounds. The bounds of the output sprite will be
	///                     the intersection of the sprite's rect and the source_rect, so the dimensions
	///                     of the output sprite may not match the `source_rect` dimensions.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
	/// # fn main() -> Result<(), RiddleError> {
	/// # let rdl =  RiddleLib::new()?; let window = WindowBuilder::new().build(rdl.context())?;
	/// let renderer = Renderer::new_from_window(&window)?;
	///
	/// // Load an image and create a sprite from it
	/// let img = Image::new(100, 100);
	/// let sprite = Sprite::new_from_image(&renderer, &img, &SpriteInitArgs::new())?;
	///
	/// // Take a portion of the sprite as a new sprite.
	/// let subsprite = sprite.subsprite(&Rect::new(vec2(75.0, 75.0), vec2(50.0, 50.0)));
	///
	/// // The subsprite dimensions will be the size of the intersection between the
	/// // source sprite and the new bounds.
	/// assert_eq!(vec2(25.0, 25.0), subsprite.dimensions());
	/// # Ok(()) }
	/// ```
	fn subsprite(&self, source_rect: &Rect<f32>) -> Self;

	/// Get the dimensions of the sprite
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
	/// # fn main() -> Result<(), RiddleError> {
	/// # let rdl =  RiddleLib::new()?; let window = WindowBuilder::new().build(rdl.context())?;
	/// let renderer = Renderer::new_from_window(&window)?;
	///
	/// // Load an image and create a sprite from it
	/// let img = Image::new(100, 100);
	/// let sprite = Sprite::new_from_image(&renderer, &img, &SpriteInitArgs::new())?;
	///
	/// // The sprite dimensions will be the same of the source image
	/// assert_eq!(vec2(100.0, 100.0), sprite.dimensions());
	/// # Ok(()) }
	/// ```
	fn dimensions(&self) -> Vector2<f32>;

	/// Render multiple sub regions of the sprite at once.
	///
	/// The regions are defined by pairs of the region of the sprite to draw in texels, and where
	/// to position the region relative to the [`SpriteRenderArgs::location`].
	///
	/// The pivot and rotation are relative to the location arg. A change in rotation will
	/// transform all rendered regions as one, not individually.
	fn render_regions<Ctx: RenderContext<R> + ?Sized>(
		&self,
		render_ctx: &mut Ctx,
		args: &SpriteRenderArgs,
		parts: &[(Rect<f32>, Vector2<f32>)],
	) -> Result<()>;

	/// Render the entire sprite.
	fn render<Ctx: RenderContext<R> + ?Sized>(
		&self,
		render_ctx: &mut Ctx,
		args: &SpriteRenderArgs,
	) -> Result<()> {
		self.render_regions(
			render_ctx,
			args,
			&[(
				Rect::new([0.0, 0.0], self.dimensions().into()),
				Vector2::new(0.0, 0.0),
			)],
		)
	}

	/// Utility function to simply render the sprite at a given location
	///
	/// See [`SpriteRenderArgs`] for how to render the sprite with more control.
	fn render_at<Ctx: RenderContext<R> + ?Sized>(
		&self,
		render_ctx: &mut Ctx,
		location: Vector2<f32>,
	) -> Result<()> {
		self.render(
			render_ctx,
			&SpriteRenderArgs {
				location,
				..Default::default()
			},
		)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
	Nearest,
	Linear,
}

impl Default for FilterMode {
	fn default() -> Self {
		FilterMode::Nearest
	}
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
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

#[derive(Clone, Debug)]
pub struct SpriteRenderArgs {
	pub location: Vector2<f32>,
	pub pivot: Vector2<f32>,
	pub scale: Vector2<f32>,
	pub angle: f32,
	pub diffuse_color: Color<f32>,
}

impl SpriteRenderArgs {
	/// New render args, with defaults, at the specified location
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
