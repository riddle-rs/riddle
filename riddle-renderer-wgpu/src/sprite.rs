use crate::{ext::*, math::*, *};

use riddle_common::Color;

/////////////////////////////////////////////////////////////////////////////
// struct Sprite
/////////////////////////////////////////////////////////////////////////////

/// A renderable region of a texture.
///
/// Multiple sprites can share a single texture. Sprites can either be built using
/// [`SpriteBuilder`], or [`SpriteAtlasBuilder`].
///
/// Use [`SpriteRenderCommand`] for access to all supported paramters when rendering
/// sprites, or use [`Sprite::render_at`] to specify only a location and use default
/// arguments for everything else.
///
/// Sprites store a reference to the [`Renderer`] which built it, which will keep
/// the renderer alive as long as the sprite is alive.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl = RiddleApp::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// // Load an image and create a sprite from it
/// let png_bytes = include_bytes!("../../example_assets/image.png");
/// let img = Image::new_from_png(std::io::Cursor::new(&png_bytes[..]))?;
/// let sprite = SpriteBuilder::new(img).build(&renderer)?;
///
/// // Render the sprite at the top left corner of the screen
/// let mut render_ctx = renderer.begin_render()?;
/// render_ctx.clear(Color::WHITE);
/// sprite.render_at(&mut render_ctx, vec2(0.0, 0.0))?;
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
pub struct Sprite {
    renderer: RendererHandle,
    texture: TextureHandle,
    source_rect: Rect<f32>,
}

impl Sprite {
    /// Construct a new sprite from an image. The image contents are copied to a texture
    /// in RGBA8 format. The entire image will be used
    pub(crate) fn new_from_image(
        renderer: &Renderer,
        img: image::Image,
        mag_filter: FilterMode,
        min_filter: FilterMode,
    ) -> Result<Sprite> {
        let texture = Texture::from_image(
            renderer.wgpu_device().device(),
            renderer.wgpu_device().queue(),
            img,
            mag_filter,
            min_filter,
            TextureType::Plain,
        )?;
        Self::from_texture(renderer, &texture)
    }

    pub(super) fn from_texture(renderer: &Renderer, texture: &Texture) -> Result<Sprite> {
        let dimensions = texture.dimensions.convert();
        Self::from_texture_with_bounds(
            renderer,
            texture,
            Rect {
                location: Vector2 { x: 0.0, y: 0.0 },
                dimensions,
            },
        )
    }

    pub(super) fn from_texture_with_bounds(
        renderer: &Renderer,
        texture: &Texture,
        source_rect: Rect<f32>,
    ) -> Result<Sprite> {
        Ok(Sprite {
            renderer: renderer.clone_handle(),
            texture: texture.clone_handle(),
            source_rect,
        })
    }

    /// Build a sprite that shares the same underlying texture but represents a different portion
    /// of the texture.
    ///
    /// # Arguments
    ///
    /// * **source_rect** - The portion of the texture that the new sprite will render, relative to
    ///                     the current sprite's bounds. The bounds of the output sprite will be
    ///                     the intersection of the sprite's rect and the source_rect, so the dimensions
    ///                     of the output sprite may not match the `source_rect` dimensions.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// # let rdl = RiddleApp::new()?; let window = WindowBuilder::new().build(rdl.context())?;
    /// let renderer = Renderer::new_from_window(&window)?;
    ///
    /// // Load an image and create a sprite from it
    /// let img = Image::new(100, 100);
    /// let sprite = SpriteBuilder::new(img).build(&renderer)?;
    ///
    /// // Take a portion of the sprite as a new sprite.
    /// let subsprite = sprite.subsprite(&Rect::new(vec2(75.0, 75.0), vec2(50.0, 50.0)));
    ///
    /// // The subsprite dimensions will be the size of the intersection between the
    /// // source sprite and the new bounds.
    /// assert_eq!(vec2(25.0, 25.0), subsprite.dimensions());
    /// # Ok(()) }
    /// ```
    pub fn subsprite(&self, source_rect: &Rect<f32>) -> Sprite {
        let mut translated_source = source_rect.clone();
        translated_source.location = translated_source.location + self.source_rect.location;

        Sprite {
            renderer: self.renderer.clone(),
            texture: self.texture.clone(),
            source_rect: self
                .source_rect
                .intersect(&translated_source)
                .unwrap_or_else(|| Rect::new(self.source_rect.location, vec2(0.0, 0.0))),
        }
    }

    pub(crate) fn render(
        &self,
        render_ctx: &mut impl RenderContext,
        args: &SpriteRenderCommand,
    ) -> Result<()> {
        let rot: glam::Mat2 = glam::Mat2::from_angle(args.angle);
        let Vector2 {
            x: tex_width,
            y: tex_height,
        } = self.texture.dimensions;

        let location: glam::Vec2 = args.location.into();
        let pivot: glam::Vec2 = args.pivot.into();

        let scale = glam::Mat2::from_scale(args.scale.into());

        let pos_topleft = glam::vec2(0.0, 0.0) - pivot;
        let pos_topright = pos_topleft + glam::vec2(self.source_rect.dimensions.x, 0.0);
        let pos_bottomleft = pos_topleft + glam::vec2(0.0, self.source_rect.dimensions.y);
        let pos_bottomright = pos_bottomleft + glam::vec2(self.source_rect.dimensions.x, 0.0);

        let uv_top = self.source_rect.location.y / (tex_height as f32);
        let uv_left = self.source_rect.location.x / (tex_width as f32);
        let uv_bottom = uv_top + (self.source_rect.dimensions.y / (tex_height as f32));
        let uv_right = uv_left + (self.source_rect.dimensions.x / (tex_width as f32));

        let color_arr: [f32; 4] = args.diffuse_color.clone().into();

        let vertex_data = [
            Vertex::ptc(
                location + (rot * (scale * pos_topleft)),
                [uv_left, uv_top],
                &color_arr,
            ),
            Vertex::ptc(
                location + (rot * (scale * pos_bottomleft)),
                [uv_left, uv_bottom],
                &color_arr,
            ),
            Vertex::ptc(
                location + (rot * (scale * pos_bottomright)),
                [uv_right, uv_bottom],
                &color_arr,
            ),
            Vertex::ptc(
                location + (rot * (scale * pos_topright)),
                [uv_right, uv_top],
                &color_arr,
            ),
        ];

        let index_data: &[u16] = &[1, 2, 0, 2, 0, 3];

        let renderable = RenderableDesc {
            texture: self.texture.clone(),
            shader: self.renderer.standard_res().default_shader.clone(),
            verts: &vertex_data[..],
            indices: index_data,
        };

        render_ctx.render_internal(&renderable)
    }

    /// Utility function to simply render the sprite at a given location
    ///
    /// This is equivalent to `SpriteRenderCommand::new(location).render(&mut ctx, &sprite)?;`.
    /// See [`SpriteRenderCommand`] for how to render the sprite with more
    /// control.
    pub fn render_at<P: Into<Vector2<f32>>>(
        &self,
        render_ctx: &mut impl RenderContext,
        location: P,
    ) -> Result<()> {
        self.render(
            render_ctx,
            &SpriteRenderCommand {
                location: location.into(),
                ..Default::default()
            },
        )
    }

    /// Get the dimensions of the sprite
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// # let rdl = RiddleApp::new()?; let window = WindowBuilder::new().build(rdl.context())?;
    /// let renderer = Renderer::new_from_window(&window)?;
    ///
    /// // Load an image and create a sprite from it
    /// let img = Image::new(100, 100);
    /// let sprite = SpriteBuilder::new(img).build(&renderer)?;
    ///
    /// // The sprite dimensions will be the same of the source image
    /// assert_eq!(vec2(100.0, 100.0), sprite.dimensions());
    /// # Ok(()) }
    /// ```
    pub fn dimensions(&self) -> Vector2<f32> {
        self.source_rect.dimensions
    }
}

/////////////////////////////////////////////////////////////////////////////
// struct SpriteRenderCommand
/////////////////////////////////////////////////////////////////////////////

/// Builder for a [`Sprite`] render call
///
/// Defaults:
///
/// * **Pivot**: `(0,0)`
/// * **Scale**: `(1,1)`
/// * **Angle**: `0`
/// * **Diffuse Color**: `Color::WHITE`
///
/// The location refers to the location of the pivot of the sprite. The pivot
/// of the sprite is relative to the top left of the sprite.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl = RiddleApp::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// # let renderer = Renderer::new_from_window(&window)?;
/// # let img = Image::new(100, 100);
/// let sprite = SpriteBuilder::new(img).build(&renderer)?;
///
/// let mut render_ctx = renderer.begin_render()?;
/// render_ctx.clear(Color::WHITE)?;
///
/// // Render the sprite
/// SpriteRenderCommand::new(vec2(0.0, 0.0))
///     .with_scale(vec2(1.0, 2.0))
///     .with_color(Color::RED)
///     .render(&mut render_ctx, &sprite)?;
///
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
pub struct SpriteRenderCommand {
    location: Vector2<f32>,
    pivot: Vector2<f32>,
    scale: Vector2<f32>,
    angle: f32,
    diffuse_color: Color<f32>,
}

impl SpriteRenderCommand {
    /// New render command with default args, at the specified location
    pub fn new<T: Into<Vector2<f32>>>(location: T) -> Self {
        let mut args = Self::default();
        args.at(location);
        args
    }

    /// Set the location of the sprite, specifying where the pivot should
    /// be placed.
    #[inline]
    pub fn at<T: Into<Vector2<f32>>>(&mut self, location: T) -> &mut Self {
        self.location = location.into();
        self
    }

    /// Set the pivot of the sprite, relative to the top left of the sprite
    #[inline]
    pub fn with_pivot<T: Into<Vector2<f32>>>(&mut self, pivot: T) -> &mut Self {
        self.pivot = pivot.into();
        self
    }

    /// Set the scale at which the sprite will be rendered
    pub fn with_scale<T: Into<Vector2<f32>>>(&mut self, scale: T) -> &mut Self {
        self.scale = scale.into();
        self
    }

    /// Set the angle at which the sprite will be rendered, in radians.
    pub fn with_angle(&mut self, angle: f32) -> &mut Self {
        self.angle = angle;
        self
    }

    /// Set the diffuse color of the sprite, which will be multiplied by the sprite
    /// colors.
    pub fn with_color(&mut self, color: Color<f32>) -> &mut Self {
        self.diffuse_color = color;
        self
    }

    /// Invoke the render command, for the given sprite, in the specified context
    pub fn render(&self, render_ctx: &mut impl RenderContext, sprite: &Sprite) -> Result<()> {
        sprite.render(render_ctx, self)
    }
}

impl Default for SpriteRenderCommand {
    fn default() -> Self {
        SpriteRenderCommand {
            location: [0.0, 0.0].into(),
            pivot: [0.0, 0.0].into(),
            angle: 0.0,
            scale: [1.0, 1.0].into(),
            diffuse_color: Color::WHITE,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// struct SpriteBuilder
/////////////////////////////////////////////////////////////////////////////

/// Builder to construct new [`Sprite`]s from `riddle_image::Image`s.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, image::*, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl = RiddleApp::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// let renderer = Renderer::new_from_window(&window)?;
///
/// let img = Image::new(100, 100);
/// let sprite = SpriteBuilder::new(img)
///     .with_filter_modes(FilterMode::Linear, FilterMode::Linear)
///     .build(&renderer)?;
/// # Ok(()) }
/// ```
pub struct SpriteBuilder {
    img: image::Image,
    mag_filter: FilterMode,
    min_filter: FilterMode,
}

impl SpriteBuilder {
    /// Create a new builder for the given image
    pub fn new(img: image::Image) -> Self {
        Self {
            img,
            mag_filter: Default::default(),
            min_filter: Default::default(),
        }
    }

    /// Specify the min and mag filters used when rendering the sprite
    pub fn with_filter_modes(mut self, mag_filter: FilterMode, min_filter: FilterMode) -> Self {
        self.mag_filter = mag_filter;
        self.min_filter = min_filter;
        self
    }

    /// Build the sprite for the given renderer
    pub fn build(self, renderer: &Renderer) -> Result<Sprite> {
        Sprite::new_from_image(renderer, self.img, self.mag_filter, self.min_filter)
    }
}
