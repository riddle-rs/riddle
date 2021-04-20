use math::Vector2;

use crate::*;

#[doc(hidden)]
pub trait WGPURenderTargetDesc<Device: WGPUDevice> {
	fn begin_render(&self) -> Result<()>;
	fn end_render(&self);
	fn renderer(&self) -> &Renderer<Device>;
	fn dimensions(&self) -> Vector2<f32>;
	fn standard_resources(&self) -> &StandardResources;
	fn with_view<F: FnOnce(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()>;
}
