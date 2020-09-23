use crate::*;

use riddle_math::Rect;

struct ImageViewDetails {
    start_offset: usize,
    end_offset: usize,
    stride: usize,
    row_len: usize,
}

pub struct ImageView<'a> {
    image: &'a Image,
    details: ImageViewDetails,
}

pub struct ImageViewMut<'a> {
    image: &'a mut Image,
    details: ImageViewDetails,
}

impl ImageViewDetails {
    fn new(image: &Image, bounds: Rect<u32>) -> Self {
        let bounds = image.rect().intersect(&bounds);

        let start_offset = ((image.width() * (bounds.location.y)) + bounds.location.x) * 4;
        let stride = image.width() * 4;
        let row_len = bounds.dimensions.x * 4;
        let end_offset = ((image.width() * (bounds.location.y + bounds.dimensions.y))
            + (bounds.location.x + bounds.location.y + 1))
            * 4;

        Self {
            start_offset: start_offset as usize,
            end_offset: end_offset as usize,
            stride: stride as usize,
            row_len: row_len as usize,
        }
    }
}

impl<'a> ImageView<'a> {
    pub(crate) fn new(image: &'a Image, bounds: Rect<u32>) -> Self {
        let details = ImageViewDetails::new(image, bounds);
        Self { image, details }
    }

    pub fn get_row_rgba8<'b>(&'b self, row: u32) -> &'b [u8]
    where
        'a: 'b,
    {
        let offset = self.details.start_offset + (self.details.stride * (row as usize));
        if offset < self.details.end_offset {
            &self.image.as_rgba8()[offset..offset + self.details.row_len]
        } else {
            &self.image.as_rgba8()[0..0]
        }
    }
}

impl<'a> ImageViewMut<'a> {
    pub(crate) fn new(image: &'a mut Image, bounds: Rect<u32>) -> Self {
        let details = ImageViewDetails::new(image, bounds);
        Self { image, details }
    }

    pub fn get_row_rgba8_mut<'b>(&'b mut self, row: u32) -> &'b mut [u8]
    where
        'a: 'b,
    {
        let offset = self.details.start_offset + (self.details.stride * (row as usize));
        if offset < self.details.end_offset {
            &mut self.image.as_rgba8_mut()[offset..offset + self.details.row_len]
        } else {
            &mut self.image.as_rgba8_mut()[0..0]
        }
    }
}
