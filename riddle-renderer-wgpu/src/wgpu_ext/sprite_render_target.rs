use crate::wgpu_ext::*;

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
pub struct WGPUSpriteRenderTarget<Device: WGPUDevice> {
    renderer: WGPURendererHandle<Device>,

    texture: WGPUTextureHandle,
    sprite: WGPUSprite<Device>,
}

impl<Device> WGPUSpriteRenderTarget<Device>
where
    Device: WGPUDevice,
{
    /// Create a new render target with the specified dimensions
    pub fn new(renderer: &WGPURenderer<Device>, dimensions: Vector2<u32>) -> Result<Self> {
        let texture = renderer.wgpu_device().with_device_info(|info| {
            Ok(WGPUTexture::new_shared(
                info.device,
                FilterMode::Linear,
                FilterMode::Linear,
                TextureType::RenderTarget,
                dimensions,
            )?)
        })?;

        let sprite = WGPUSprite::from_texture(renderer, &texture)?;

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
    pub fn begin_render(&self) -> Result<impl RenderContext + '_> {
        let encoder = self.renderer.wgpu_device().with_device_info(|info| {
            Ok(info
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }))
        })?;
        BufferedRenderer::new(self, encoder)
    }

    /// Get the sprite which can be used to render the contents of the render target.
    pub fn sprite(&self) -> &WGPUSprite<Device> {
        &self.sprite
    }
}

impl<'a, Device> WGPURenderTargetDesc<'a, Device> for &'a WGPUSpriteRenderTarget<Device>
where
    Device: WGPUDevice,
{
    fn dimensions(&self) -> Vector2<f32> {
        self.sprite.dimensions()
    }

    #[inline]
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

    fn renderer(&self) -> &WGPURenderer<Device> {
        &self.renderer
    }

    fn standard_resources(&self) -> &StandardResources {
        self.renderer.standard_res()
    }

    fn begin_render(&self) -> Result<()> {
        Ok(())
    }

    fn end_render(&self) {}
}
