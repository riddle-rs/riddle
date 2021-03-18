use crate::wgpu_ext::*;

#[doc(hidden)]
pub trait WGPURenderable {
	fn with_renderable<R, F: FnOnce(&WGPURenderableDesc) -> Result<R>>(&self, f: F) -> Result<R>;
}

#[doc(hidden)]
pub struct WGPURenderableDesc<'a> {
	pub(crate) texture: WGPUTextureHandle,
	pub(crate) shader: WGPUShaderHandle,
	pub(crate) verts: &'a [Vertex],
	pub(crate) indices: &'a [u16],
}

impl<'a> WGPURenderable for WGPURenderableDesc<'a> {
	fn with_renderable<R, F>(&self, f: F) -> Result<R>
	where
		F: FnOnce(&WGPURenderableDesc) -> Result<R>,
	{
		f(self)
	}
}
