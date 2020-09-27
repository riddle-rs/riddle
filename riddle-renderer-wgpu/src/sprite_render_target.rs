use crate::{ext::*, math::*, *};

/// A target which can be both rendered to and referenced as a [`Sprite`] for rendering.
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
/// let target = SpriteRenderTarget::new(&renderer, vec2(100, 100))?;
///
/// let mut target_ctx = target.begin_render()?;
/// target_ctx.clear(Color::BLUE)?;
/// target_ctx.present()?;
///
/// let mut render_ctx = renderer.begin_render()?;
/// render_ctx.clear(Color::GREEN)?;
/// target.sprite().render_at(&mut render_ctx, vec2(0.0, 0.0))?;
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
pub struct SpriteRenderTarget {
    renderer: RendererHandle,

    texture: TextureHandle,
    sprite: Sprite,
}

impl SpriteRenderTarget {
    /// Create a new render target with the specified dimensions
    pub fn new(renderer: &Renderer, dimensions: Vector2<u32>) -> Result<SpriteRenderTarget> {
        let texture = Texture::new(
            &renderer.wgpu_device().device(),
            FilterMode::Linear,
            FilterMode::Linear,
            TextureType::RenderTarget,
            dimensions,
        )?;

        let sprite = Sprite::from_texture(renderer, &texture)?;

        Ok(Self {
            renderer: renderer.clone_handle(),

            texture,
            sprite,
        })
    }

    /// Get a render context for the current swap chain frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// # let rdl =  RiddleLib::new()?;
    /// # let window = WindowBuilder::new().build(rdl.context())?;
    /// # let renderer = Renderer::new_from_window(&window)?;
    /// let target = SpriteRenderTarget::new(&renderer, vec2(100, 100))?;
    ///
    /// let mut target_ctx = target.begin_render()?;
    /// target_ctx.clear(Color::RED);
    /// target_ctx.present();
    /// # Ok(()) }
    /// ```
    pub fn begin_render<'a>(&'a self) -> Result<impl RenderContext + 'a> {
        let encoder = self
            .renderer
            .wgpu_device()
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        BufferedRenderer::new(self, encoder)
    }

    /// Get the sprite which can be used to render the contents of the render target.
    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}

impl<'a> RenderTargetDesc<'a> for &'a SpriteRenderTarget {
    fn dimensions(&self) -> Vector2<f32> {
        self.sprite.dimensions()
    }

    fn with_view<F: FnOnce(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()> {
        let view = self
            .texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(wgpu::TextureFormat::Bgra8UnormSrgb),
                dimension: Some(wgpu::TextureViewDimension::D2),
                aspect: wgpu::TextureAspect::All,
                ..Default::default()
            });
        f(&view)
    }

    fn wgpu_device(&self) -> &dyn ext::RendererWGPUDevice {
        self.renderer.wgpu_device()
    }

    fn standard_resources(&self) -> &StandardResources {
        self.renderer.standard_res()
    }

    fn begin_render(&self) -> Result<()> {
        Ok(())
    }

    fn end_render(&self) {}
}
