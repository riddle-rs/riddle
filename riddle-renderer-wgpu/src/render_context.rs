use crate::{math::*, *};

pub trait RenderTargetDesc<'a> {
    fn begin_render(&self) -> Result<()>;
    fn end_render(&self);
    fn wgpu_device(&self) -> &dyn ext::RendererWGPUDevice;
    fn dimensions(&self) -> Vector2<f32>;
    fn standard_resources(&self) -> &StandardResources;
    fn with_view<F: FnMut(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()>;
}

pub trait RenderContext {
    fn set_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<()>;
    fn push_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<()>;
    fn pop_transform(&mut self) -> Result<()>;
    fn clear(&mut self, color: Color<f32>) -> Result<()>;
    fn render<R: Renderable>(&mut self, renderable: &R) -> Result<()>;
    fn fill_rect(&mut self, rect: &Rect<f32>, color: [f32; 4]) -> Result<()>;
    fn present(self) -> Result<()>;
}

pub struct RenderableDesc<'a> {
    pub(crate) texture: TextureHandle,
    pub(crate) shader: ShaderHandle,
    pub(crate) verts: &'a [Vertex],
    pub(crate) indices: &'a [u16],
}

impl<'a> Renderable for RenderableDesc<'a> {
    fn with_renderable<R, F>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&RenderableDesc) -> Result<R>,
    {
        f(self)
    }
}

pub trait Renderable {
    fn with_renderable<R, F: FnOnce(&RenderableDesc) -> Result<R>>(&self, f: F) -> Result<R>;
}
