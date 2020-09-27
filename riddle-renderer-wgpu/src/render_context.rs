use crate::{math::*, *};

#[doc(hidden)]
pub trait RenderTargetDesc<'a> {
    fn begin_render(&self) -> Result<()>;
    fn end_render(&self);
    fn wgpu_device(&self) -> &dyn ext::RendererWGPUDevice;
    fn dimensions(&self) -> Vector2<f32>;
    fn standard_resources(&self) -> &StandardResources;
    fn with_view<F: FnMut(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()>;
}

/// Types which accept render calls, tracks current world transform, and are consumed
/// when the calls are presented.
///
/// # Example
///
/// ```no_run
/// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
/// # fn main() -> Result<(), RiddleError> {
/// # let rdl =  RiddleLib::new()?;
/// # let window = WindowBuilder::new().build(rdl.context())?;
/// # let renderer = Renderer::new_from_window(&window)?;
/// let mut render_ctx /*: impl RenderContext*/ = renderer.begin_render()?;
///
/// render_ctx.clear(Color::RED)?;
///
/// // Change the current transform matrix, and draw a rect
/// render_ctx.set_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;
/// render_ctx.fill_rect(&Rect::new(vec2(0.0, 0.0), vec2(10.0, 10.0)), Color::GREEN)?;
///
/// render_ctx.present()?;
/// # Ok(()) }
/// ```
pub trait RenderContext {
    /// Replace the current world transform.
    fn set_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<()>;

    /// Fill the target with a flat color.
    fn clear(&mut self, color: Color<f32>) -> Result<()>;

    /// Render a `Renderable` to the target with the current world transform.
    ///
    /// This is only called by internal crate code.
    fn render_internal<R: Renderable>(&mut self, renderable: &R) -> Result<()>;

    /// Draw a solid rect with the given color.
    fn fill_rect(&mut self, rect: &Rect<f32>, color: Color<f32>) -> Result<()>;

    /// Consume the context and present any outstanding draw calls.
    fn present(self) -> Result<()>;
}

#[doc(hidden)]
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

#[doc(hidden)]
pub trait Renderable {
    fn with_renderable<R, F: FnOnce(&RenderableDesc) -> Result<R>>(&self, f: F) -> Result<R>;
}
