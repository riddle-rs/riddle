use crate::{math::*, *};

pub trait RenderTargetDesc<'a> {
    fn renderer(&self) -> &Renderer;
    fn dimensions(&self) -> Vector2<f32>;
    fn with_view<R, F: FnOnce(&wgpu::TextureView) -> Result<R, RendererError>>(
        &self,
        f: F,
    ) -> Result<R, RendererError>;
}

pub trait RenderContext {
    fn set_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<(), RendererError>;
    fn push_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<(), RendererError>;
    fn pop_transform(&mut self) -> Result<(), RendererError>;
    fn clear(&mut self, color: Color<f32>) -> Result<(), RendererError>;
    fn render<R: Renderable>(&mut self, renderable: &R) -> Result<(), RendererError>;
    fn fill_rect(&mut self, rect: &Rect<f32>, color: [f32; 4]) -> Result<(), RendererError>;
    fn present(self) -> Result<(), RendererError>;
}

pub struct RenderableDesc<'a> {
    pub(crate) texture: TextureHandle,
    pub(crate) shader: ShaderHandle,
    pub(crate) verts: &'a [Vertex],
    pub(crate) indices: &'a [u16],
}

impl<'a> Renderable for RenderableDesc<'a> {
    fn with_renderable<R, F>(&self, f: F) -> Result<R, RendererError>
    where
        F: FnOnce(&RenderableDesc) -> Result<R, RendererError>,
    {
        f(self)
    }
}

pub trait Renderable {
    fn with_renderable<R, F: FnOnce(&RenderableDesc) -> Result<R, RendererError>>(
        &self,
        f: F,
    ) -> Result<R, RendererError>;
}
