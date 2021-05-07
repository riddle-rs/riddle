use crate::*;

use riddle_font::ImgFont;
use riddle_math::{Rect, SpacialNumericConversion, Vector2};

/// An efficient [`riddle_font::ImgFont`] renderer.
///
/// Use [`crate::SpriteRenderArgs`] for access to all supported paramters when rendering
/// sprites.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, font::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// // Load a TTFont, generate an ImgFont, and then construct the SpriteFont with it
/// let img_font: ImgFont = {
///     let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
///     let ttfont = TtFont::load(&font_bytes[..])?;
///     ImgFontGenerator::new("abcdefghijklmnopqrstuvwxyz ", 32).generate(&ttfont)?
/// };
/// let sprite_font = SpriteFont::new(&renderer, img_font)?;
///
/// // Render the sprite at the top left corner of the screen
/// renderer.render(|render_ctx| {
///     render_ctx.clear(Color::WHITE);
///     sprite_font.render(
///         render_ctx,
///         &SpriteRenderArgs::new([0.0, 0.0]).with_color(Color::BLACK),
///         "hello world",
///     )
/// })?;
/// # Ok(()) }
/// ```
pub struct SpriteFont<R: CommonRenderer> {
	sprite: R::Sprite,
	font: ImgFont,
}

impl<R: CommonRenderer> SpriteFont<R> {
	/// Build a SpriteFont from the ImgFont given. The ImgFont's image will get loaded in to a
	/// texture, and its glyph information will be used for layout.
	pub fn new(renderer: &R, font: ImgFont) -> Result<Self, R::Error> {
		let sprite = R::Sprite::new_from_image(
			renderer,
			font.image(),
			&SpriteInitArgs {
				min_filter: FilterMode::Linear,
				mag_filter: FilterMode::Linear,
			},
		)?;

		Ok(Self { sprite, font })
	}

	pub fn render<Ctx: RenderContext<R> + ?Sized>(
		&self,
		render_ctx: &mut Ctx,
		render_args: &SpriteRenderArgs,
		text: &str,
	) -> std::result::Result<(), R::Error> {
		let mut parts: Vec<(Rect<f32>, Vector2<f32>)> = Vec::with_capacity(text.len());
		self.font.layout(text, |_, rect, location| {
			parts.push((rect.clone().convert(), location.convert()));
		});
		self.sprite
			.render_regions(render_ctx, render_args, &parts[..])
	}
}
