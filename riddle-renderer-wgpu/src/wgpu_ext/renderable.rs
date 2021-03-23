use crate::wgpu_ext::*;

#[doc(hidden)]
pub struct WGPURenderableDesc<'a> {
	pub(crate) texture: WGPUTextureHandle,
	pub(crate) shader: WGPUShaderHandle,
	pub(crate) verts: &'a [Vertex],
	pub(crate) indices: &'a [u16],
}
