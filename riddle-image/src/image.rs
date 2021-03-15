use crate::*;

use riddle_common::{Color, ColorElementConversion};
use riddle_math::*;

use futures::{AsyncRead, AsyncReadExt};
use std::io::{BufReader, Cursor, Read, Write};

/// A representation of an image stored in main memory. The image is stored
/// as RGBA32.
#[derive(Clone, Debug)]
pub struct Image {
    img: ::image::RgbaImage,
}

impl Image {
    /// Load an image from a `Read` instance which emits image file data in the
    /// specified format.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; fn main() -> Result<(), ImageError> {
    /// let png_bytes = include_bytes!("../../example_assets/image.png");
    /// let png_img = Image::load(&png_bytes[..], ImageFormat::Png)?;
    /// # Ok(()) }
    /// ```
    pub fn load<R: Read>(mut r: R, format: ImageFormat) -> Result<Self> {
        let mut buf = vec![];
        r.read_to_end(&mut buf)?;
        Self::from_bytes(&buf, format)
    }

    /// Load an image from a `AsyncRead` instance which emits image file data in the
    /// specified format.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; fn main() -> Result<(), ImageError> { futures::executor::block_on(async_main()) }
    /// # async fn async_main() -> Result<(), ImageError> {
    /// let png_bytes = include_bytes!("../../example_assets/image.png");
    /// let png_img = Image::load_async(&png_bytes[..], ImageFormat::Png).await?;
    /// # Ok(()) }
    /// ```
    pub async fn load_async<R>(mut data: R, format: ImageFormat) -> Result<Self>
    where
        R: AsyncRead + Unpin,
    {
        let mut buf = vec![];
        data.read_to_end(&mut buf).await?;
        Self::from_bytes(&buf, format)
    }

    /// Save an image to a `Write` instance, emitting image file data in the specified format.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; fn main() -> Result<(), ImageError> {
    /// let img = Image::new(4,4);
    /// let buf: Vec<u8> = vec![];
    /// img.save(buf, ImageFormat::Png)?;
    /// # Ok(()) }
    /// ```
    pub fn save<W: Write>(&self, mut w: W, format: ImageFormat) -> Result<()> {
        match format {
            ImageFormat::Png => {
                ::image::png::PngEncoder::new(w).encode(
                    self.as_rgba8(),
                    self.width(),
                    self.height(),
                    ::image::ColorType::Rgba8,
                )?;
            }
            ImageFormat::Bmp => {
                ::image::bmp::BmpEncoder::new(&mut w).encode(
                    self.as_rgba8(),
                    self.width(),
                    self.height(),
                    ::image::ColorType::Rgba8,
                )?;
            }
            ImageFormat::Jpeg => {
                ::image::jpeg::JpegEncoder::new(&mut w).encode(
                    self.as_rgba8(),
                    self.width(),
                    self.height(),
                    ::image::ColorType::Rgba8,
                )?;
            }
        }
        Ok(())
    }

    /// Load an image from a byte slice in the specified format.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; fn main() -> Result<(), ImageError> {
    /// let png_bytes = include_bytes!("../../example_assets/image.png");
    /// let png_img = Image::from_bytes(&png_bytes[..], ImageFormat::Png)?;
    /// # Ok(()) }
    /// ```
    pub fn from_bytes(bytes: &[u8], format: ImageFormat) -> Result<Self> {
        let buf_reader = BufReader::new(Cursor::new(bytes));
        let img = match format {
            ImageFormat::Png => {
                ::image::DynamicImage::from_decoder(::image::png::PngDecoder::new(buf_reader)?)?
            }
            ImageFormat::Bmp => {
                ::image::DynamicImage::from_decoder(::image::bmp::BmpDecoder::new(buf_reader)?)?
            }
            ImageFormat::Jpeg => {
                ::image::DynamicImage::from_decoder(::image::jpeg::JpegDecoder::new(buf_reader)?)?
            }
        };
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
    /// assert_eq!(Color::rgba(0,0,0,0), img.get_pixel([0, 0]));
    /// ```
    pub fn get_pixel<L: Into<Vector2<u32>>>(&self, location: L) -> Color<u8> {
        let location = location.into();
        let c: ::image::Rgba<u8> = *self.img.get_pixel(location.x, location.y);
        Color::rgba(c[0], c[1], c[2], c[3])
    }

    /// Set the color of the pixel at the given coordinates
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*;
    /// let mut img = Image::new(1,1);
    /// img.set_pixel([0, 0], Color::rgba(1.0, 0.0, 0.0, 1.0));
    /// assert_eq!(Color::rgba(255,0,0,255), img.get_pixel([0, 0]));
    /// ```
    pub fn set_pixel<L: Into<Vector2<u32>>, C: ColorElementConversion<Color<u8>>>(
        &mut self,
        location: L,
        color: C,
    ) {
        let color: Color<u8> = color.convert();
        let color: [u8; 4] = color.into();
        let location = location.into();
        self.img.put_pixel(location.x, location.y, color.into());
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
    /// assert_eq!(Color::rgba(255, 0, 0, 0), img.get_pixel([0, 0]));
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
    /// source.set_pixel([0, 0], Color::<u8>::RED);
    ///
    /// let mut dest = Image::new(2,1);
    /// dest.blit(&source, Vector2::new(1, 0));
    ///
    /// assert_eq!(Color::ZERO, dest.get_pixel([0, 0]));
    /// assert_eq!(Color::RED, dest.get_pixel([1, 0]));
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

    /// Fill a rect portion of the image with a given color.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; use riddle_math::*;
    /// let mut img = Image::new(2,2);
    /// img.fill_rect(Rect::new([0, 0], [2, 1]), Color::<u8>::RED);
    /// img.fill_rect(Rect::new([1, 0], [1, 2]), Color::<u8>::GREEN);
    ///
    /// assert_eq!(Color::RED, img.get_pixel([0, 0]));
    /// assert_eq!(Color::ZERO, img.get_pixel([0, 1]));
    /// assert_eq!(Color::GREEN, img.get_pixel([1, 0]));
    /// assert_eq!(Color::GREEN, img.get_pixel([1, 1]));
    /// ```
    pub fn fill_rect<C: ColorElementConversion<Color<u8>>>(&mut self, rect: Rect<u32>, color: C) {
        if let Some(dest_rect) = self.rect().intersect(&rect) {
            let color_bytes: [u8; 4] = color.convert().into();
            let mut row_vec = Vec::with_capacity(dest_rect.dimensions.x as usize * 4);
            for _ in 0..dest_rect.dimensions.x {
                row_vec.extend_from_slice(&color_bytes[..]);
            }

            let mut dest_view = self.create_view_mut(dest_rect.clone().convert());
            for row_idx in 0..(dest_rect.dimensions.y as u32) {
                let dest = dest_view.get_row_rgba8_mut(row_idx);
                dest.clone_from_slice(bytemuck::cast_slice(&row_vec[..]));
            }
        }
    }

    /// Fill the entire image with a certain color.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_image::*; use riddle_math::*;
    /// let mut img = Image::new(2,2);
    /// img.fill(Color::<u8>::RED);
    ///
    /// assert_eq!(Color::RED, img.get_pixel([0, 0]));
    /// assert_eq!(Color::RED, img.get_pixel([1, 1]));
    /// ```
    pub fn fill<C: ColorElementConversion<Color<u8>>>(&mut self, color: C) {
        self.fill_rect(self.rect(), color)
    }

    pub(crate) fn create_view(&self, rect: Rect<u32>) -> ImageView {
        ImageView::new(self, rect)
    }

    pub(crate) fn create_view_mut(&mut self, rect: Rect<u32>) -> ImageViewMut {
        ImageViewMut::new(self, rect)
    }
}

impl image_ext::ImageImageExt for Image {
    fn image_rgbaimage(&self) -> &::image::RgbaImage {
        &self.img
    }

    fn image_from_dynimage(img: ::image::DynamicImage) -> Self {
        Self {
            img: img.into_rgba(),
        }
    }
}

/// The set of support image file formats which [`Image`] can load
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ImageFormat {
    Png,
    Bmp,
    Jpeg,
}
