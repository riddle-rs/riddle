use riddle_common::Color;
use riddle_image::{packer::ImagePackerSizePolicy, Image, ImagePacker};
use riddle_math::{Rect, SpacialNumericConversion, Vector2};
use std::collections::{HashMap, HashSet};

use crate::rusttype_ext::*;
use crate::*;

/// Represents an image font, which is an Image containing glyphs, glyph information
/// specifying which parts of the image map to which glyph, and layout information.
pub struct ImgFont {
	img: Image,
	glyphs: HashMap<char, ImgFontGlyph>,
	vertical_spacing: u32,
}

impl ImgFont {
	/// This represents the distance, in pixels, between the top of one line of text and the
	/// next line of text.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let ttf_font = TtFont::load(&font_bytes[..])?;
	/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
	///
	/// assert!(img_font.vertical_spacing() > 0);
	/// # Ok(()) }
	/// ```
	pub fn vertical_spacing(&self) -> u32 {
		self.vertical_spacing
	}

	/// The image storing the glyph image data.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let ttf_font = TtFont::load(&font_bytes[..])?;
	/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
	///
	/// assert!(img_font.image().width() > 0);
	/// # Ok(()) }
	/// ```
	pub fn image(&self) -> &Image {
		&self.img
	}

	/// The mapping from [`char`] to glyphs registered with the font.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let ttf_font = TtFont::load(&font_bytes[..])?;
	/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
	///
	/// assert_eq!(6, img_font.glyphs().len());
	/// assert_eq!('A', img_font.glyphs().get(&'A').unwrap().character);
	/// # Ok(()) }
	/// ```
	pub fn glyphs(&self) -> &HashMap<char, ImgFontGlyph> {
		&self.glyphs
	}

	/// Render a string to a [`riddle_image::Image`] using the font at its native scale.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let ttf_font = TtFont::load(&font_bytes[..])?;
	/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
	///
	/// let rendered_img = img_font.render_simple("AABCA")?;
	///
	/// assert_eq!(rendered_img.height(), img_font.vertical_spacing());
	/// assert!(rendered_img.width() > 0);
	/// # Ok(()) }
	/// ```
	pub fn render_simple(&self, text: &str) -> Result<Image> {
		let total_width = text
			.chars()
			.map(|c| {
				if let Some(glyph) = self.glyphs.get(&c) {
					glyph.horizontal_spacing
				} else {
					0
				}
			})
			.sum();

		let mut output_image = Image::new(total_width, self.vertical_spacing);
		let mut current_x = 0_u32;

		for c in text.chars() {
			if let Some(glyph) = self.glyphs.get(&c) {
				if let Some(source_rect) = &glyph.rect {
					output_image.blit_rect(
						&self.img,
						source_rect,
						Vector2::new(current_x as i32, 0) + glyph.placement_offset,
					);
				}
				current_x += glyph.horizontal_spacing;
			}
		}

		self.layout(text, |_, source_rect, location| {
			output_image.blit_rect(&self.img, source_rect, location.convert());
		});

		Ok(output_image)
	}

	/// Layout a string using the font, relative to a (0,0) point representing the top left of the
	/// layout space.
	///
	/// The callback fuction is invoked once for each character, providing the part of the image to
	/// be used, and the location to draw that image.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_font::*;
	/// # fn main() -> Result<(), FontError> {
	/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
	/// let ttf_font = TtFont::load(&font_bytes[..])?;
	/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
	///
	/// let mut last_x = 0;
	/// let mut total_width = 0;
	/// let mut string = String::from("");
	///
	/// img_font.layout("AABCA", |c, rect, location| {
	///     assert!(location.x >= last_x);
	///     last_x = location.x;
	///     total_width += rect.dimensions.x;
	///     string += c.to_string().as_str();
	/// });
	///
	/// assert!(last_x > 0);
	/// assert!(total_width > 0);
	/// assert_eq!("AABCA", string);
	/// # Ok(()) }
	/// ```
	pub fn layout<F: FnMut(char, &Rect<u32>, Vector2<u32>)>(&self, text: &str, mut f: F) {
		let mut current_x = 0_u32;

		for c in text.chars() {
			if let Some(glyph) = self.glyphs.get(&c) {
				if let Some(source_rect) = &glyph.rect {
					let position: Vector2<u32> =
						Vector2::new(current_x, 0) + glyph.placement_offset.convert();
					f(c, &source_rect, position);
				}
				current_x += glyph.horizontal_spacing;
			}
		}
	}
}

/// Details describing a glyph stored in an image font
#[derive(Debug, Clone, Default)]
pub struct ImgFontGlyph {
	/// The character that the glyph maps to.
	pub character: char,

	/// The part of the imgfont that contains this character.
	pub rect: Option<Rect<u32>>,

	/// Offset relative to to the expected location the glyph should be placed, useful if the rect
	/// is a different size than the horizontal or vertical spacing of the glyph/font.
	pub placement_offset: Vector2<i32>,

	/// The horizontal spacing between the left of this glyph and the left of the next glyph
	pub horizontal_spacing: u32,
}

/// Layout will happen relative to a (0,0) point representing the top left of the layout space.
pub struct ImgFontLayoutRule {
	/// The maximum width the space.
	pub max_width: Option<u32>,
}

/// A utility to construct an [`ImgFont`] given an image and a collection of glyph data.
///
/// # Example
///
/// ```
/// # use riddle_font::*; use riddle_image::*; use riddle_math::*;
/// # fn main() -> Result<(), FontError> {
/// let img = Image::new(64, 64);
/// let mut img_font_builder = ImgFontBuilder::new();
/// img_font_builder.vertical_spacing(32);
///
/// img_font_builder.with_glyph(ImgFontGlyph {
///     character: 'a',
///     rect: Some(Rect::new([0,0], [32, 32])),
///     horizontal_spacing: 32,
///     .. Default::default()
/// });
///
/// let img_font = img_font_builder.build(img);
///
/// assert_eq!(128, img_font.render_simple("aaaa")?.width());
/// # Ok(()) }
/// ```
#[derive(Default, Debug, Clone)]
pub struct ImgFontBuilder {
	glyphs: Vec<ImgFontGlyph>,
	vertical_spacing: u32,
}

impl ImgFontBuilder {
	/// Construct a new builder.
	pub fn new() -> Self {
		Self::default()
	}

	/// Set the vertical spacing of the font. See [`ImgFont::vertical_spacing`].
	pub fn vertical_spacing(&mut self, spacing: u32) -> &mut Self {
		self.vertical_spacing = spacing;
		self
	}

	/// Register a glyph with the font. If more that one glyph is added with the same value for
	/// [`ImgFontGlyph::character`] the last one added will be the one that is respected.
	pub fn with_glyph(&mut self, glyph: ImgFontGlyph) -> &mut Self {
		self.glyphs.push(glyph);
		self
	}

	/// Take ownership of the given image, and finalize building of the [`ImgFont`].
	pub fn build(self, img: Image) -> ImgFont {
		ImgFont {
			img,
			glyphs: self
				.glyphs
				.iter()
				.map(|g| (g.character, g.clone()))
				.collect(),
			vertical_spacing: self.vertical_spacing,
		}
	}
}

/// A utility for generating [`ImgFont`]s from [`TtFont`]s.
///
/// # Example
///
///
/// ```
/// # use riddle_font::*;
/// # fn main() -> Result<(), FontError> {
/// # let font_bytes = include_bytes!("../../example_assets/Roboto-Regular.ttf");
/// let ttf_font = TtFont::load(&font_bytes[..])?;
///
/// let img_font = ImgFontGenerator::new("ABCDEF", 32).generate(&ttf_font)?;
///
/// assert_eq!(6, img_font.glyphs().len());
/// # Ok(()) }
/// ```
#[derive(Default, Clone)]
pub struct ImgFontGenerator {
	characters: HashSet<char>,
	pixel_height: u32,
	fore_color: Color<u8>,
	packing_size_policy: ImagePackerSizePolicy,
}

impl ImgFontGenerator {
	/// Create a new generator, specifying the set of characters to generate and the pixel height at
	/// which the font will be rendered.
	pub fn new(chars: &str, pixel_height: u32) -> Self {
		Self {
			characters: chars.chars().collect(),
			pixel_height,
			fore_color: Color::WHITE,
			..Default::default()
		}
	}

	/// Override the packing rule which controls how the individual glyph images are combined in to
	/// the final image. See [`riddle_image::packer::ImagePackerSizePolicy`].
	///
	/// The default is [`riddle_image::packer::ImagePackerSizePolicy::Pow2Square`].
	pub fn packing_size_policy(&mut self, policy: ImagePackerSizePolicy) -> &mut Self {
		self.packing_size_policy = policy;
		self
	}

	/// Build the ImgFont for the given [`TtFont`] using the parameters specified in the generator.
	pub fn generate(&self, ttfont: &TtFont) -> Result<ImgFont> {
		let characters: Vec<char> = self.characters.iter().copied().collect();
		let mut images: Vec<Image> = vec![];
		let mut imgfont_glyphs: Vec<ImgFontGlyph> = vec![];

		let font = ttfont.rustype_font();
		let scale = rusttype::Scale {
			x: self.pixel_height as f32,
			y: self.pixel_height as f32,
		};

		let v_metrics = font.v_metrics(scale);
		let char_height = v_metrics.ascent - v_metrics.descent;
		let vertical_spacing = char_height + v_metrics.line_gap;

		for c in characters.iter() {
			let glyph = font
				.glyph(*c)
				.scaled(rusttype::Scale {
					x: self.pixel_height as f32,
					y: self.pixel_height as f32,
				})
				.positioned(rusttype::Point {
					x: 0.0,
					y: v_metrics.ascent,
				});
			let h_metrics = glyph.unpositioned().h_metrics();

			let (image, img_glyph) = match glyph.pixel_bounding_box() {
				Some(pixel_bounds) => {
					let mut image =
						Image::new(pixel_bounds.width() as u32, pixel_bounds.height() as u32);
					glyph.draw(|x, y, v| {
						let mut c = self.fore_color.clone();
						c.a = (255.0 * v) as u8;
						image.set_pixel([x as u32, y as u32], c);
					});
					let img_glyph = ImgFontGlyph {
						character: *c,
						horizontal_spacing: h_metrics.advance_width as u32,
						placement_offset: Vector2::new(pixel_bounds.min.x, pixel_bounds.min.y),
						rect: None,
					};
					(image, img_glyph)
				}
				_ => {
					let image = Image::new(0, 0);
					let img_glyph = ImgFontGlyph {
						character: *c,
						horizontal_spacing: h_metrics.advance_width as u32,
						placement_offset: Vector2::new(0, 0),
						rect: None,
					};
					(image, img_glyph)
				}
			};

			images.push(image);
			imgfont_glyphs.push(img_glyph);
		}

		let img_list: Vec<&Image> = images.iter().collect();
		let packer = ImagePacker::new()
			.padding(2)
			.size_policy(self.packing_size_policy.clone())
			.pack(&img_list[..])?;

		let mut img_font_builder = ImgFontBuilder::new();
		img_font_builder.vertical_spacing(vertical_spacing as u32);

		for (packed_rect, img_glyph) in packer.rects().iter().zip(imgfont_glyphs.iter()) {
			let mut glyph = img_glyph.clone();
			glyph.rect = if packed_rect.dimensions == Vector2::new(0, 0) {
				None
			} else {
				Some(packed_rect.clone())
			};

			img_font_builder.with_glyph(glyph);
		}

		Ok(img_font_builder.build(packer.take_image()))
	}
}
