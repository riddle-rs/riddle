use math::SpacialNumericConversion;

use crate::*;

/// Construct a set of [`Sprite`]s from a set of [`riddle_image::Image`]s which share a texture atlas.
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
/// let img1 = Image::new(100, 100);
/// let img2 = Image::new(50, 50);
///
/// let mut sprite1 = None;
/// let mut sprite2 = None;
///
/// SpriteAtlasBuilder::new()
///     .with_image(img1, &mut sprite1)
///     .with_image(img2, &mut sprite2)
///     .build(&renderer)?;
///
/// assert!(sprite1.is_some());
/// assert!(sprite2.is_some());
/// # Ok(()) }
/// ```
pub struct SpriteAtlasBuilder<'a, Device: WgpuDevice> {
	images: Vec<(image::Image, &'a mut Option<Sprite<Device>>)>,

	mag_filter: FilterMode,
	min_filter: FilterMode,
}

impl<'a, Device> SpriteAtlasBuilder<'a, Device>
where
	Device: WgpuDevice,
{
	/// A new empty atlas builder
	pub fn new() -> Self {
		Self {
			images: vec![],
			mag_filter: Default::default(),
			min_filter: Default::default(),
		}
	}

	/// Add an image to be packed in to the atlas, along with a reference
	/// to the `Option<Sprite>` which will store the sprite when the atlas is built.
	pub fn with_image(mut self, img: image::Image, sprite: &'a mut Option<Sprite<Device>>) -> Self {
		self.images.push((img, sprite));
		self
	}

	/// Specify the min and mag filters used when rendering the created sprites.
	pub fn with_filter_modes(mut self, mag_filter: FilterMode, min_filter: FilterMode) -> Self {
		self.mag_filter = mag_filter;
		self.min_filter = min_filter;
		self
	}

	/// Construct the atlas texture from the given set of images, and update the
	/// `Option<Sprite>` references.
	pub fn build(self, renderer: &Renderer<Device>) -> Result<()> {
		let SpriteAtlasBuilder {
			mut images,
			mag_filter,
			min_filter,
		} = self;

		let packing_images: Vec<&image::Image> = images.iter().map(|(img, _)| img).collect();
		let packed = image::ImagePacker::new()
			.pack(&packing_images[..])
			.map_err(|_| WgpuRendererError::Unknown)?;

		let texture = renderer.wgpu_device().with_device_info(|info| {
			Ok(Texture::from_image(
				info.device,
				info.queue,
				packed.image(),
				mag_filter,
				min_filter,
				TextureType::Plain,
			))
		})?;

		for ((_, output), rect) in images.iter_mut().zip(packed.rects().iter()) {
			**output = Some(Sprite::from_texture_with_bounds(
				renderer,
				&texture,
				rect.clone().convert(),
			)?);
		}

		Ok(())
	}
}

impl<'a, Device> Default for SpriteAtlasBuilder<'a, Device>
where
	Device: WgpuDevice,
{
	fn default() -> Self {
		Self::new()
	}
}
