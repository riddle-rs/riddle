use math::Vector2;

use crate::*;

pub(crate) struct SwapChainFrameTarget<'a> {
    renderer: &'a Renderer,
    frame: wgpu::SwapChainFrame,
    dimensions: Vector2<f32>,
}

impl<'a> SwapChainFrameTarget<'a> {
    pub fn new(
        renderer: &'a Renderer,
        frame: wgpu::SwapChainFrame,
        dimensions: Vector2<f32>,
    ) -> Self {
        Self {
            renderer: renderer,
            frame,
            dimensions,
        }
    }
}

impl<'a> RenderTargetDesc<'a> for SwapChainFrameTarget<'a> {
    fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    fn dimensions(&self) -> Vector2<f32> {
        self.dimensions
    }

    fn with_view<R, F: FnOnce(&wgpu::TextureView) -> Result<R, RendererError>>(
        &self,
        f: F,
    ) -> Result<R, RendererError> {
        f(&self.frame.output.view)
    }
}
