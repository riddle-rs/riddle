use crate::*;

use riddle_math::*;

use riddle_common::{Color, ColorElementConversion};
use std::io::{Read, Seek};

/// A representation of an image stored in RAM. Can be loaded from and saved to
/// a variety of formats.
#[derive(Clone)]
pub struct Image {
    img: ::image::RgbaImage,
}

impl Image {
    pub fn new_from_png<R: Read + Seek>(r: R) -> Result<Self, ImageError> {
        let img = ::image::load(std::io::BufReader::new(r), ::image::ImageFormat::Png)
            .map_err(|_| ImageError::Unknown)?;
        Ok(Image {
            img: img.into_rgba(),
        })
    }

    pub fn new(width: u32, height: u32) -> Result<Self, ImageError> {
        let img =
            ::image::RgbaImage::from_raw(width, height, vec![0u8; (width * height * 4) as usize])
                .ok_or(ImageError::Unknown)?;
        Ok(Image { img })
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color<u8> {
        let c: ::image::Rgba<u8> = self.img.get_pixel(x, y).clone();
        Color::rgba(c[0], c[1], c[2], c[3])
    }

    pub fn set_pixel<C: ColorElementConversion<Color<u8>>>(&mut self, x: u32, y: u32, color: C) {
        let color: Color<u8> = color.convert();
        let color: [u8; 4] = color.into();
        self.img.put_pixel(x, y, color.into());
    }

    pub fn as_rgba8(&self) -> &[u8] {
        self.img.as_ref()
    }

    pub fn as_rgba8_mut(&mut self) -> &mut [u8] {
        self.img.as_mut()
    }

    pub fn byte_count(&self) -> usize {
        self.img.as_ref().len()
    }

    pub fn width(&self) -> u32 {
        self.img.width()
    }

    pub fn height(&self) -> u32 {
        self.img.height()
    }

    pub fn dimensions(&self) -> Vector2<u32> {
        let (w, h) = self.img.dimensions();
        Vector2 { x: w, y: h }
    }

    pub fn rect(&self) -> Rect<u32> {
        Rect {
            location: Vector2 { x: 0, y: 0 },
            dimensions: self.dimensions(),
        }
    }

    pub fn create_view<'a>(&'a self, rect: Rect<u32>) -> ImageView<'a> {
        ImageView::new(self, rect)
    }

    pub fn create_view_mut<'a>(&'a mut self, rect: Rect<u32>) -> ImageViewMut<'a> {
        ImageViewMut::new(self, rect)
    }

    pub fn blit(&mut self, other: &Image, location: Vector2<i32>) {
        let (dest_rect, src_rect) =
            Rect::intersect_relative_to_both(self.dimensions(), other.dimensions(), location);

        let mut dest_view = self.create_view_mut(dest_rect.clone().convert());
        let src_view = other.create_view(src_rect.convert());

        for row in 0..(dest_rect.dimensions.y as u32) {
            let dest = dest_view.get_row_rgba8_mut(row);
            let src = src_view.get_row_rgba8(row);

            dest.clone_from_slice(src);
        }
    }
}
