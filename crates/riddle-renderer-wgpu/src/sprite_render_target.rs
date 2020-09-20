use crate::{math::*, *};

pub struct SpriteRenderTarget {
    renderer: RendererHandle,

    texture: TextureHandle,
    sprite: Sprite,
}

impl<'a> RenderTargetDesc<'a> for &'a SpriteRenderTarget {
    fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    fn dimensions(&self) -> Vector2<f32> {
        self.sprite.dimensions()
    }

    fn with_view<R, F: FnOnce(&wgpu::TextureView) -> Result<R, RendererError>>(
        &self,
        f: F,
    ) -> Result<R, RendererError> {
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
}

impl SpriteRenderTarget {
    pub fn new(
        renderer: &Renderer,
        dimensions: Vector2<u32>,
    ) -> Result<SpriteRenderTarget, RendererError> {
        let texture = Texture::new(
            &renderer.device,
            FilterMode::Linear,
            FilterMode::Linear,
            TextureType::RenderTarget,
            dimensions,
        )?;

        let sprite = Sprite::from_texture(renderer, &texture)?;

        Ok(Self {
            renderer: renderer.clone_handle().unwrap(),

            texture,
            sprite,
        })
    }

    pub fn begin_render<'a>(&'a self) -> impl RenderContext + 'a {
        let encoder = self
            .renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        StreamRenderer::new(self, encoder)
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}
