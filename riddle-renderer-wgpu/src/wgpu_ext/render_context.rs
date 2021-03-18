use crate::wgpu_ext::*;

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
	fn render_internal<R: WGPURenderable>(&mut self, renderable: &R) -> Result<()>;

	/// Draw a solid rect with the given color.
	fn fill_rect(&mut self, rect: &Rect<f32>, color: Color<f32>) -> Result<()>;

	/// Consume the context and present any outstanding draw calls.
	fn present(self) -> Result<()>;
}
