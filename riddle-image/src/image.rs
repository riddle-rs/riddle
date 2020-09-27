use crate::*;

use riddle_math::*;

use riddle_common::{Color, ColorElementConversion};
use std::io::{Read, Seek};

/// A representation of an image stored in main memory. The image is stored
/// as RGBA32.
#[derive(Clone)]
pub struct Image {
    img: ::image::RgbaImage,
}

impl Image {
    /// Load an image from a `Read + Seek` instance which emits png file data.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; fn main() -> Result<(), ImageError> {
    /// let png_bytes = include_bytes!("../../example_assets/image.png");
    /// let png_img = Image::new_from_png(std::io::Cursor::new(&png_bytes[..]))?;
    /// # Ok(()) }
    /// ```
    pub fn new_from_png<R: Read + Seek>(r: R) -> Result<Self> {
        let img = ::image::load(std::io::BufReader::new(r), ::image::ImageFormat::Png)?;
        Ok(Image {
            img: img.into_rgba(),
        })
    }

    /// Create a new image with the given dimensions, all pixels are initialized
    /// to 0x00000000.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// // Create a single pixel image
    /// let img = Image::new(1,1);
    /// ```
    pub fn new(width: u32, height: u32) -> Self {
        let img =
            ::image::RgbaImage::from_raw(width, height, vec![0u8; (width * height * 4) as usize])
                .unwrap();
        Image { img }
    }

    /// Get the color of the pixel at the given coordinates
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(Color::rgba(0,0,0,0), img.get_pixel(0, 0));
    /// ```
    pub fn get_pixel(&self, x: u32, y: u32) -> Color<u8> {
        let c: ::image::Rgba<u8> = self.img.get_pixel(x, y).clone();
        Color::rgba(c[0], c[1], c[2], c[3])
    }

    /// Set the color of the pixel at the given coordinates
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let mut img = Image::new(1,1);
    /// img.set_pixel(0, 0, Color::rgba(1.0, 0.0, 0.0, 1.0));
    /// assert_eq!(Color::rgba(255,0,0,255), img.get_pixel(0, 0));
    /// ```
    pub fn set_pixel<C: ColorElementConversion<Color<u8>>>(&mut self, x: u32, y: u32, color: C) {
        let color: Color<u8> = color.convert();
        let color: [u8; 4] = color.into();
        self.img.put_pixel(x, y, color.into());
    }

    /// Borrow the bytes representing the entire image, encoded as RGBA8
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(0x00u8, img.as_rgba8()[0]);
    /// ```
    pub fn as_rgba8(&self) -> &[u8] {
        self.img.as_ref()
    }

    /// Mutably borrow the bytes representing the entire image, encoded as RGBA8
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let mut img = Image::new(1,1);
    /// let bytes = img.as_rgba8_mut();
    /// bytes[0] = 0xFF;
    /// assert_eq!(Color::rgba(255, 0, 0, 0), img.get_pixel(0,0));
    /// ```
    pub fn as_rgba8_mut(&mut self) -> &mut [u8] {
        self.img.as_mut()
    }

    /// Get the byte count of the entire image encoded as RGBA8
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(4, img.byte_count());
    /// ```
    pub fn byte_count(&self) -> usize {
        self.img.as_ref().len()
    }

    /// Width of the image in pixels
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(1, img.width());
    /// ```
    pub fn width(&self) -> u32 {
        self.img.width()
    }

    /// Height of the image in pixels
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(1, img.height());
    /// ```
    pub fn height(&self) -> u32 {
        self.img.height()
    }

    /// Dimension of the image in pixels
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; use riddle_math::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(Vector2::new(1, 1), img.dimensions());
    /// ```
    pub fn dimensions(&self) -> Vector2<u32> {
        let (w, h) = self.img.dimensions();
        Vector2 { x: w, y: h }
    }

    /// Get the bounding rect for the image, located at (0,0) and having size
    /// equal to the image's dimensions.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; use riddle_math::*;
    /// let img = Image::new(1,1);
    /// assert_eq!(Rect::new(Vector2::new(0, 0), Vector2::new(1, 1)), img.rect());
    /// ```
    pub fn rect(&self) -> Rect<u32> {
        Rect {
            location: Vector2 { x: 0, y: 0 },
            dimensions: self.dimensions(),
        }
    }

    /// Blit another image on to self. The location is the relative offset of the (0,0) pixel of the
    /// source image relative to self's (0,0) pixel.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; use riddle_math::*;
    /// let mut source = Image::new(1,1);
    /// source.set_pixel(0, 0, Color::<u8>::RED);
    ///
    /// let mut dest = Image::new(2,1);
    /// dest.blit(&source, Vector2::new(1, 0));
    ///
    /// assert_eq!(Color::ZERO, dest.get_pixel(0,0));
    /// assert_eq!(Color::RED, dest.get_pixel(1, 0));
    /// ```
    pub fn blit(&mut self, source: &Image, location: Vector2<i32>) {
        if let Some((dest_rect, src_rect)) =
            Rect::intersect_relative_to_both(self.dimensions(), source.dimensions(), location)
        {
            let mut dest_view = self.create_view_mut(dest_rect.clone().convert());
            let src_view = source.create_view(src_rect.convert());

            for row in 0..(dest_rect.dimensions.y as u32) {
                let dest = dest_view.get_row_rgba8_mut(row);
                let src = src_view.get_row_rgba8(row);

                dest.clone_from_slice(src);
            }
        }
    }

    pub(crate) fn create_view<'a>(&'a self, rect: Rect<u32>) -> ImageView<'a> {
        ImageView::new(self, rect)
    }

    pub(crate) fn create_view_mut<'a>(&'a mut self, rect: Rect<u32>) -> ImageViewMut<'a> {
        ImageViewMut::new(self, rect)
    }
}
