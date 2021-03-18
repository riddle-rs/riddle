use crate::wgpu_ext::*;

use riddle_font::ImgFont;

/// An efficient [`riddle_font::ImgFont`] renderer.
///
/// Use [`crate::SpriteRenderArgs`] for access to all supported paramters when rendering
/// sprites.
///
/// Sprite fonts store a reference to the [`Renderer`] which built it, which will keep
/// the renderer alive as long as the sprite font is alive.
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
///     let font_bytes = include_bytes!("../../../example_assets/Roboto-Regular.ttf");
///     let ttfont = TTFont::load(&font_bytes[..])?;
///     ImgFontGenerator::new("abcdefghijklmnopqrstuvwxyz ", 32).generate(&ttfont)?
/// };
/// let sprite_font = SpriteFont::new(&renderer, img_font)?;
///
/// // Render the sprite at the top left corner of the screen
/// let mut render_ctx = renderer.begin_render()?;
/// render_ctx.clear(Color::WHITE);
/// sprite_font.render(
///     &mut render_ctx,
///     &SpriteRenderArgs::new([0.0, 0.0]).with_color(Color::BLACK),
///     "hello world",
/// )?;
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
pub struct WGPUSpriteFont<Device: WGPUDevice> {
    sprite: WGPUSprite<Device>,
    font: ImgFont,
}

impl<Device: WGPUDevice> WGPUSpriteFont<Device> {
    /// Build a SpriteFont from the ImgFont given. The ImgFont's image will get loaded in to a
    /// texture, and its glyph information will be used for layout.
    pub fn new(renderer: &WGPURenderer<Device>, font: ImgFont) -> Result<Self> {
        let sprite = WGPUSprite::new_from_image(
            renderer,
            font.image(),
            FilterMode::Linear,
            FilterMode::Linear,
        )?;

        Ok(Self { sprite, font })
    }

    pub fn render(
        &self,
        render_ctx: &mut impl RenderContext,
        render_args: &SpriteRenderArgs,
        text: &str,
    ) -> Result<()> {
        let mut parts: Vec<(Rect<f32>, Vector2<f32>)> = Vec::with_capacity(text.len());
        self.font.layout(text, |_, rect, location| {
            parts.push((rect.clone().convert(), location.convert()));
        });
        self.sprite
            .render_regions(render_ctx, render_args, &parts[..])
    }
}
