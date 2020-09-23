use crate::{ext::*, math::Vector2, *};

pub(crate) struct SwapChainFrameTarget<'a> {
    renderer: &'a Renderer,
    dimensions: Vector2<f32>,
}

impl<'a> SwapChainFrameTarget<'a> {
    pub fn new(renderer: &'a Renderer, dimensions: Vector2<f32>) -> Self {
        Self {
            renderer: renderer,
            dimensions,
        }
    }
}

impl<'a> RenderTargetDesc<'a> for SwapChainFrameTarget<'a> {
    #[inline]
    fn dimensions(&self) -> Vector2<f32> {
        self.dimensions
    }

    #[inline]
    fn with_view<F: FnMut(&wgpu::TextureView) -> Result<()>>(&self, mut f: F) -> Result<()> {
        self.renderer
            .wgpu_device()
            .with_frame(&mut |frame| f(&frame.output.view))
    }

    fn wgpu_device(&self) -> &dyn ext::RendererWGPUDevice {
        self.renderer.wgpu_device()
    }

    fn standard_resources(&self) -> &StandardResources {
        self.renderer.standard_res()
    }

    fn begin_render(&self) -> Result<()> {
        self.wgpu_device().begin_frame()
    }

    fn end_render(&self) {
        self.wgpu_device().end_frame()
    }
}
