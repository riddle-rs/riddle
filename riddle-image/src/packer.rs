use crate::*;

use riddle_math::{Rect, SpacialNumericConversion, Vector2};

use thiserror::Error;

/// Utility for packing multiple images to a single image.
///
/// # Example
///
/// ```
/// # use riddle_image::*; use riddle_math::*;
/// let mut img1 = Image::new(2,2);
/// let mut img2 = Image::new(3,3);
/// let packed = ImagePacker::new().pack(&[&img1, &img2]).unwrap();
///
/// assert!(packed.image().dimensions().x > 3 || packed.image().dimensions().y > 3);
/// assert_eq!(img1.dimensions(), packed.rects()[0].dimensions);
/// assert_eq!(img2.dimensions(), packed.rects()[1].dimensions);
/// ```
#[derive(Default)]
pub struct ImagePacker {
	size_policy: ImagePackerSizePolicy,
	padding: u32,
}

impl ImagePacker {
	/// Create the ImagePacker, with default settings.
	///
	/// * Size Policy: [`ImagePackerSizePolicy::Pow2Square`]
	/// * Padding: 0
	pub fn new() -> Self {
		ImagePacker::default()
	}

	/// Set the size policy for the packer. This controls how the packing algorithm sizes
	/// the output image, and how those dimensions grow if the algorithm needs to grow the output
	/// image.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_image::{*, packer::*}; use riddle_math::*;
	/// let mut img1 = Image::new(2,2);
	/// let mut img2 = Image::new(3,3);
	/// let packed = ImagePacker::new()
	///     .size_policy(ImagePackerSizePolicy::Fixed(Vector2::new(10, 10)))
	///     .pack(&[&img1, &img2]).unwrap();
	///
	/// assert_eq!(Vector2::new(10, 10), packed.image().dimensions());
	/// ```
	pub fn size_policy(&mut self, policy: ImagePackerSizePolicy) -> &mut Self {
		self.size_policy = policy;
		self
	}

	/// The padding around each image in pixels.The gap between images will thus be twice the
	/// padding value.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_image::{*, packer::*}; use riddle_math::*;
	/// let mut img1 = Image::new(2,2);
	/// let packed = ImagePacker::new()
	///     .size_policy(ImagePackerSizePolicy::Pow2Square)
	///     .padding(1)
	///     .pack(&[&img1]).unwrap();
	///
	/// assert_eq!(Vector2::new(4,4), packed.image().dimensions());
	/// assert_eq!(Vector2::new(1,1), packed.rects()[0].location);
	/// ```
	pub fn padding(&mut self, amount: u32) -> &mut Self {
		self.padding = amount;
		self
	}

	/// Pack the slice of images provides in to a single image using the settings stored in the
	/// image packer. The values in [`ImagePackerResult::rects()`] will reference images in the same
	/// order as the slice provided.
	///
	/// If the slice is empty an Error result will be returned
	///
	/// # Example
	///
	/// ```
	/// # use riddle_image::*; use riddle_math::*;
	/// let mut img1 = Image::new(2,2);
	/// let mut img2 = Image::new(3,3);
	/// let packed = ImagePacker::new()
	///     .pack(&[&img1, &img2]).unwrap();
	///
	/// assert_eq!(Vector2::new(2, 2), packed.rects()[0].dimensions);
	/// assert_eq!(Vector2::new(3, 3), packed.rects()[1].dimensions);
	/// ```
	pub fn pack(
		&self,
		images: &[&Image],
	) -> std::result::Result<ImagePackerResult, ImagePackerError> {
		if images.is_empty() {
			return Err(ImagePackerError::NoSourceImages);
		}

		let mut sorted_images: Vec<(usize, &Image)> = images
			.iter()
			.enumerate()
			.map(|(i, img)| (i, *img))
			.collect();
		sorted_images
			.sort_by(|(_, a), (_, b)| (b.height() * b.width()).cmp(&(a.height() * a.width())));

		let mut current_size = self.size_policy.initial_size();

		'PACK_WITH_SIZE: loop {
			let mut rects = vec![Rect::<u32>::default(); images.len()];

			let mut output_image = Image::new(current_size.x, current_size.y);
			let mut occupancy_image = Image::new(current_size.x, current_size.y);

			let mut current_y = 0;
			let mut current_x = 0;

			for (image_idx, image) in sorted_images.iter() {
				let padded_size = if image.dimensions() == Vector2::new(0, 0) {
					Vector2::new(0, 0)
				} else {
					let padded_size = Vector2::new(
						image.width() + (self.padding * 2),
						image.height() + (self.padding * 2),
					);

					'FIND_GAP: while current_y < output_image.height() {
						let mut gap_length = 0;
						while current_x < output_image.width() {
							if occupancy_image.get_pixel([current_x, current_y])
								== Color::<u8>::ZERO
							{
								gap_length += 1;
							} else {
								gap_length = 0;
							}

							current_x += 1;
							if gap_length == padded_size.x {
								current_x -= padded_size.x;
								break 'FIND_GAP;
							}
						}
						current_x = 0;
						current_y += 1;
					}
					padded_size
				};

				if current_y + padded_size.y > output_image.height() {
					current_size = self
						.size_policy
						.increase_size(current_size)
						.ok_or(ImagePackerError::UnableToFitImages)?;
					continue 'PACK_WITH_SIZE;
				}

				let blit_rect = Rect::new(
					Vector2::new(current_x + self.padding, current_y + self.padding),
					image.dimensions(),
				);
				let occupancy_rect = Rect::new([current_x, current_y].into(), padded_size);

				occupancy_image.fill_rect(occupancy_rect, Color::<u8>::WHITE);
				output_image.blit(image, blit_rect.location.convert());
				rects[*image_idx] = blit_rect;
			}

			return Ok(ImagePackerResult {
				image: output_image,
				rects,
			});
		}
	}
}

/// Packed images result, combining the generated image and the locations of all the images which
/// were packed.
///
/// See [`ImagePacker`] for how to pack images and create this result.
#[derive(Debug)]
pub struct ImagePackerResult {
	image: Image,
	rects: Vec<Rect<u32>>,
}

impl ImagePackerResult {
	/// Borrow the image containing the packed image results.
	pub fn image(&self) -> &Image {
		&self.image
	}

	/// Take the image, dropping the result.
	pub fn take_image(self) -> Image {
		self.image
	}

	/// The locations of each of the supplied [`Image`]s in the order in which they were supplied to
	/// [`ImagePacker::pack`].
	pub fn rects(&self) -> &Vec<Rect<u32>> {
		&self.rects
	}
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ImagePackerError {
	#[error("No source images supplied")]
	NoSourceImages,

	#[error("Unable to fit all images in to packed image")]
	UnableToFitImages,
}

/// Controls the initial size of the output packed image size, and how that image grows over time
/// if more space is required to pack all the supplied images.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImagePackerSizePolicy {
	/// The output image will initially be the supplied size, and if the images can't be packed in
	/// to that size packing will fail.
	Fixed(Vector2<u32>),

	/// The output image will initially be the supplied value. If the images can't be packed in to
	/// that size, the output image will be increased by the same amount. The resultant image will
	/// always be `n * initial_dimensions`.
	LinearSteps(Vector2<u32>),

	/// The output image will initially be `[1, 1]`. If the images can't be packed in to that size
	/// the dimensions will double. The resultablt image will always be a square with side of `2^n`.
	Pow2Square,
}

impl ImagePackerSizePolicy {
	fn initial_size(&self) -> Vector2<u32> {
		match self {
			Self::Fixed(size) => *size,
			Self::LinearSteps(step) => *step,
			Self::Pow2Square => Vector2::new(1, 1),
		}
	}

	fn increase_size(&self, current_size: Vector2<u32>) -> Option<Vector2<u32>> {
		match self {
			Self::Fixed(_) => None,
			Self::LinearSteps(step) => Some(current_size + *step),
			Self::Pow2Square => Some(current_size * 2),
		}
	}
}

impl Default for ImagePackerSizePolicy {
	fn default() -> Self {
		Self::Pow2Square
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use itertools::*;

	fn assert_valid_result(results: &ImagePackerResult) {
		// All images are disjoint
		for rects in results.rects().iter().combinations(2) {
			assert_eq!(None, rects[0].intersect(rects[1]));
		}

		// All images are in the output image
		for rect in results.rects() {
			assert_eq!(Some(rect.clone()), results.image().rect().intersect(rect));
		}
	}

	#[test]
	fn pow2square_pack_0x0_image() {
		let img = Image::new(0, 0);

		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(Vector2::new(0, 0), packed.rects()[0].dimensions);
	}

	#[test]
	fn pow2square_pack_0x0_with_large_image() {
		let img1 = Image::new(0, 0);
		let mut img2 = Image::new(3, 3);
		img2.fill(Color::<u8>::RED);

		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img1, &img2])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(Vector2::new(0, 0), packed.rects()[0].dimensions);
		assert_eq!(Vector2::new(4, 4), packed.image().dimensions());
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[1].location)
		);
	}

	#[test]
	fn pow2square_pack_single_pixel_image() {
		let mut img = Image::new(1, 1);
		img.fill(Color::<u8>::RED);

		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[0].location)
		);
	}

	#[test]
	fn pow2square_pack_3x3_in_pow2_square() {
		let mut img = Image::new(3, 3);
		img.fill(Color::<u8>::RED);
		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[0].location)
		);
	}

	#[test]
	fn pow2square_pack_single_big_multiple_small() {
		let mut img1 = Image::new(3, 3);
		img1.fill(Color::<u8>::RED);
		let mut img2 = Image::new(1, 1);
		img2.fill(Color::<u8>::BLUE);
		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img1, &img2, &img2, &img2, &img2])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[0].location)
		);
		assert_eq!(
			Color::<u8>::BLUE,
			packed.image().get_pixel(packed.rects()[1].location)
		);
	}

	#[test]
	fn pow2square_pack_multiple_big_multiple_small() {
		let mut img1 = Image::new(3, 3);
		img1.fill(Color::<u8>::RED);
		let mut img2 = Image::new(4, 4);
		img2.fill(Color::<u8>::BLUE);
		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Pow2Square)
			.pack(&[&img1, &img2])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[0].location)
		);
		assert_eq!(
			Color::<u8>::BLUE,
			packed.image().get_pixel(packed.rects()[1].location)
		);
	}

	#[test]
	fn fixed_pack_multiple_big_multiple_small() {
		let mut img1 = Image::new(3, 3);
		img1.fill(Color::<u8>::RED);
		let mut img2 = Image::new(4, 4);
		img2.fill(Color::<u8>::BLUE);
		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Fixed(Vector2::new(10, 10)))
			.pack(&[&img1, &img2])
			.unwrap();

		assert_valid_result(&packed);
		assert_eq!(
			Color::<u8>::RED,
			packed.image().get_pixel(packed.rects()[0].location)
		);
		assert_eq!(
			Color::<u8>::BLUE,
			packed.image().get_pixel(packed.rects()[1].location)
		);
	}

	#[test]
	fn fixed_pack_too_small() {
		let mut img1 = Image::new(3, 3);
		img1.fill(Color::<u8>::RED);
		let mut img2 = Image::new(4, 4);
		img2.fill(Color::<u8>::BLUE);
		let packed = ImagePacker::new()
			.size_policy(ImagePackerSizePolicy::Fixed(Vector2::new(4, 4)))
			.pack(&[&img1, &img2]);

		assert_eq!(ImagePackerError::UnableToFitImages, packed.unwrap_err());
	}
}
