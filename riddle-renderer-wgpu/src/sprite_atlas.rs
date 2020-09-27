use crate::{ext::*, math::*, *};

use std::rc::Rc;

/// Construct a set of [`Sprite`]s from a set of `riddle_image::Image`s which share a texture atlas.
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
pub struct SpriteAtlasBuilder<'a> {
    images: Vec<(image::Image, &'a mut Option<Sprite>)>,

    mag_filter: FilterMode,
    min_filter: FilterMode,

    max_height: u32,
    total_width: u32,
}

impl<'a> SpriteAtlasBuilder<'a> {
    /// A new empty atlas builder
    pub fn new() -> Self {
        Self {
            images: vec![],
            max_height: 0,
            total_width: 0,
            mag_filter: Default::default(),
            min_filter: Default::default(),
        }
    }

    /// Add an image to be packed in to the atlas, along with a reference
    /// to the `Option<Sprite>` which will store the sprite when the atlas is built.
    pub fn with_image(mut self, img: image::Image, sprite: &'a mut Option<Sprite>) -> Self {
        self.total_width += img.width();
        self.max_height = self.max_height.max(img.height());
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
    pub fn build(self, renderer: &Renderer) -> Result<()> {
        let mut atlas = image::Image::new(self.total_width, self.max_height);
        let mut sprite_bounds = vec![];
        let mut x = 0;
        for (img, sprite) in self.images {
            sprite_bounds.push((
                Rect {
                    location: Vector2 { x: x, y: 0 },
                    dimensions: img.dimensions(),
                },
                sprite,
            ));
            atlas.blit(&img, Vector2 { x, y: 0 }.convert());
            x += img.width();
        }

        let texture = Rc::new(Texture::from_image(
            renderer.wgpu_device().device(),
            renderer.wgpu_device().queue(),
            atlas,
            self.mag_filter,
            self.min_filter,
            TextureType::Plain,
        )?);

        for (bounds, sprite) in sprite_bounds {
            *sprite = Some(Sprite::from_texture_with_bounds(
                renderer,
                &texture,
                bounds.convert(),
            )?);
        }

        Ok(())
    }
}
