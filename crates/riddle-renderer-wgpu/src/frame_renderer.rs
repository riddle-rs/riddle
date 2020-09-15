use math::Rect;

use crate::*;

pub struct FrameRenderer {
    renderer: RendererHandle,

    pub(crate) stream_renderer: StreamRenderer,
}

impl FrameRenderer {
    pub(crate) fn new(
        renderer: &Renderer,
        frame: wgpu::SwapChainFrame,
        encoder: wgpu::CommandEncoder,
    ) -> Self {
        FrameRenderer {
            renderer: renderer.clone_handle().unwrap(),
            stream_renderer: StreamRenderer::new(renderer, encoder, frame),
        }
    }

    pub fn renderer(&self) -> &Renderer {
        &self.renderer
    }

    /// Set the clear color and mark the frame buffer for clearing. The actual clear operation
    /// will be performed when the next batched render happens, or when `present` is called,
    /// whichever comes first.
    pub fn clear(&mut self, color: Color<f32>) -> Result<(), RendererError> {
        self.stream_renderer.clear(color)
    }

    pub fn set_transform(
        &mut self,
        transform: mint::ColumnMatrix4<f32>,
    ) -> Result<(), RendererError> {
        self.stream_renderer.set_transform(transform)
    }

    pub fn push_transform(
        &mut self,
        transform: mint::ColumnMatrix4<f32>,
    ) -> Result<(), RendererError> {
        self.stream_renderer.push_transform(transform)
    }

    pub fn pop_transform(&mut self) -> Result<(), RendererError> {
        self.stream_renderer.pop_transform()
    }

    pub fn fill_rect(&mut self, rect: &Rect<f32>, color: [f32; 4]) -> Result<(), RendererError> {
        self.stream_renderer.fill_rect(rect, color)
    }

    pub fn present(self) -> Result<(), RendererError> {
        self.stream_renderer.present()
    }

    pub(crate) fn render(
        &mut self,
        args: &StreamRenderArgs,
        verts: &[Vertex],
        indices: &[u16],
    ) -> Result<(), RendererError> {
        self.stream_renderer.stream_render(args, verts, indices)
    }
}
