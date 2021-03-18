use crate::{math::Vector2, wgpu_ext::*};

pub(crate) struct SwapChainFrameTarget<'a, Device: WGPUDevice> {
	renderer: &'a WGPURenderer<Device>,
	dimensions: Vector2<f32>,
}

impl<'a, Device: WGPUDevice> SwapChainFrameTarget<'a, Device> {
	pub fn new(renderer: &'a WGPURenderer<Device>, dimensions: Vector2<f32>) -> Self {
		Self {
			renderer,
			dimensions,
		}
	}
}

impl<'a, Device: WGPUDevice> WGPURenderTargetDesc<'a, Device> for SwapChainFrameTarget<'a, Device> {
	#[inline]
	fn dimensions(&self) -> Vector2<f32> {
		self.dimensions
	}

	#[inline]
	fn with_view<F>(&self, f: F) -> Result<()>
	where
		F: FnOnce(&wgpu::TextureView) -> Result<()>,
	{
		self.renderer
			.wgpu_device()
			.with_frame(|frame| f(&frame.output.view))
	}

	fn renderer(&self) -> &WGPURenderer<Device> {
		&self.renderer
	}

	fn standard_resources(&self) -> &StandardResources {
		self.renderer.standard_res()
	}

	fn begin_render(&self) -> Result<()> {
		self.renderer().wgpu_device().begin_frame()
	}

	fn end_render(&self) {
		self.renderer().wgpu_device().end_frame()
	}
}
