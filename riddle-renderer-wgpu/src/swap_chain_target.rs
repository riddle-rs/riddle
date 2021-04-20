use crate::{math::Vector2, *};

pub struct SwapChainFrameTarget<Device: WGPUDevice> {
	renderer: Renderer<Device>,
	dimensions: Vector2<f32>,
}

impl<Device: WGPUDevice> SwapChainFrameTarget<Device> {
	pub fn new(renderer: &Renderer<Device>, dimensions: Vector2<f32>) -> Self {
		Self {
			renderer: renderer.clone(),
			dimensions,
		}
	}
}

impl<Device: WGPUDevice> WGPURenderTargetDesc<Device> for SwapChainFrameTarget<Device> {
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

	fn renderer(&self) -> &Renderer<Device> {
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
