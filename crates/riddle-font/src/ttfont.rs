use crate::*;

use riddle_common::Color;
use riddle_image::Image;

use std::io::Read;

pub struct TTFont {
    font: rusttype::Font<'static>,
}

impl TTFont {
    /// Construct a new TTFont from a `Read` instance.
    pub fn new<R: Read>(mut r: R) -> Result<Self> {
        let mut data = vec![0u8; 0];
        r.read_to_end(&mut data)
            .map_err(|e| CommonError::IOError(e))?;

        let font = rusttype::Font::try_from_vec(data).ok_or(FontError::FontParseFailed)?;
        Ok(TTFont { font })
    }

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

            let mut img = Image::new(max_x as u32, 45);

            for glyph in layout {
                let bb = glyph.pixel_bounding_box().unwrap_or_default();
                glyph.draw(|x, y, v| {
                    let b = (255.0 * v) as u8;
                    img.set_pixel(
                        (bb.min.x + x as i32) as u32,
                        (base_line + bb.min.y + (y as i32)) as u32,
                        Color::rgba(255, 255, 255, b),
                    );
                })
            }

            Ok(img)
        }
    }
}
