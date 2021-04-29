use crate::*;

use riddle_common::Color;
use riddle_image::Image;

use futures::{AsyncRead, AsyncReadExt};
use std::io::Read;

/// Represents a parsed TTF file, and facilitates simple rendering
pub struct TtFont {
	font: rusttype::Font<'static>,
}

impl TtFont {
	/// Construct a new TtFont from a `Read` instance. The source will be read to
	/// the end, and the entire buffer parsed as a TTF font file.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// let ttf_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let font = TtFont::load(&ttf_bytes[..])?;
	/// # Ok (()) }
	/// ```
	pub fn load<R: Read>(mut r: R) -> Result<Self> {
		let mut data = vec![0u8; 0];
		r.read_to_end(&mut data)?;

		let font = rusttype::Font::try_from_vec(data).ok_or(FontError::FontParseFailed)?;
		Ok(TtFont { font })
	}

	/// Construct a new TtFont from an `AsyncRead` instance. The source will be read
	/// the end, and the entire buffer parsed as a TTF font file.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*; fn main() -> Result<(), FontError> { futures::executor::block_on(async_main()) }
	/// # async fn async_main() -> Result<(), FontError> {
	/// let ttf_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let font = TtFont::load_async(&ttf_bytes[..]).await?;
	/// # Ok(()) }
	/// ```
	pub async fn load_async<R: AsyncRead + Unpin>(mut r: R) -> Result<Self> {
		let mut data = vec![0u8; 0];
		r.read_to_end(&mut data).await?;

		let font = rusttype::Font::try_from_vec(data).ok_or(FontError::FontParseFailed)?;
		Ok(TtFont { font })
	}

	/// Render a string in this font to an image. It will only be a single line of text, even
	/// if newlines are present.
	///
	/// # Arguments
	///
	/// * `text` - The string to be rendered, as a single line of text.
	/// * `pixel_height` - The font size (in pixels) to render the font in. The output image will have
	///                    the same height.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let ttf_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let font = TtFont::load(&ttf_bytes[..])?;
	/// let image = font.render_simple("A String", 24)?;
	/// # Ok (()) }
	/// ```
	pub fn render_simple(&self, text: &str, pixel_height: u32) -> Result<Image> {
		let scale = rusttype::Scale::uniform(pixel_height as f32);
		let layout: Vec<rusttype::PositionedGlyph> = self
			.font
			.layout(text, scale, rusttype::Point { x: 0.0, y: 0.0 })
			.collect();

		if layout.is_empty() {
			Ok(Image::new(0, 0))
		} else {
			let base_line = self.font.v_metrics(scale).ascent as i32;

			let max_x = layout
				.iter()
				.map(|glyph| glyph.pixel_bounding_box().unwrap_or_default().max.x)
				.max()
				.unwrap();

			let mut img = Image::new(max_x as u32, pixel_height);

			for glyph in layout {
				let bb = glyph.pixel_bounding_box().unwrap_or_default();
				glyph.draw(|x, y, v| {
					let b = (255.0 * v) as u8;
					img.set_pixel(
						[
							(bb.min.x + x as i32) as u32,
							(base_line + bb.min.y + (y as i32)) as u32,
						],
						Color::rgba(255, 255, 255, b),
					);
				})
			}

			Ok(img)
		}
	}
}

impl rusttype_ext::RustTypeTtFontExt for TtFont {
	fn rustype_font(&self) -> &rusttype::Font<'static> {
		&self.font
	}
}
