use crate::wgpu_ext::*;

#[doc(hidden)]
pub trait WGPURenderTargetDesc<'a, Device: WGPUDevice> {
    fn begin_render(&self) -> Result<()>;
    fn end_render(&self);
    fn renderer(&self) -> &WGPURenderer<Device>;
    fn dimensions(&self) -> Vector2<f32>;
    fn standard_resources(&self) -> &StandardResources;
    fn with_view<F: FnOnce(&wgpu::TextureView) -> Result<()>>(&self, f: F) -> Result<()>;
}
