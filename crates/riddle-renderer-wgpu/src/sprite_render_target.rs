use crate::{ext::*, math::*, *};

pub struct SpriteRenderTarget {
    renderer: RendererHandle,

    texture: TextureHandle,
    sprite: Sprite,
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

impl SpriteRenderTarget {
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

    pub fn begin_render<'a>(&'a self) -> Result<impl RenderContext + 'a> {
        let encoder = self
            .renderer
            .wgpu_device()
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        BufferedRenderer::new(self, encoder)
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}
