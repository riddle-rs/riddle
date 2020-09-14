use crate::*;

use math::Rect;
use wgpu::util::DeviceExt;

#[derive(Clone)]
pub(crate) struct StreamRenderArgs {
    pub texture: <Texture as CloneHandle>::Handle,
    pub shader: <Shader as CloneHandle>::Handle,
}

impl PartialEq for StreamRenderArgs {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.texture, &other.texture)
            && std::sync::Arc::ptr_eq(&self.shader, &other.shader)
    }
}

impl Eq for StreamRenderArgs {}

pub(crate) struct StreamRenderer {
    renderer: <Renderer as CloneHandle>::Handle,
    current_args: Option<StreamRenderArgs>,

    verts: Vec<Vertex>,
    indices: Vec<u16>,

    pending_clear_color: Option<[f32; 4]>,
    view_matrix_stack: Vec<mint::ColumnMatrix4<f32>>,

    encoder: wgpu::CommandEncoder,
    frame: wgpu::SwapChainFrame,
}

impl StreamRenderer {
    pub fn new(
        renderer: &Renderer,
        encoder: wgpu::CommandEncoder,
        frame: wgpu::SwapChainFrame,
    ) -> Self {
        let identity: mint::ColumnMatrix4<f32> = glam::Mat4::identity().into();
        Self {
            renderer: renderer.clone_handle().unwrap(),
            current_args: None,
            verts: vec![],
            indices: vec![],
            pending_clear_color: None,
            view_matrix_stack: vec![identity],
            encoder,
            frame,
        }
    }

    pub fn set_transform(
        &mut self,
        transform: mint::ColumnMatrix4<f32>,
    ) -> Result<(), RendererError> {
        self.flush()?;
        match self.view_matrix_stack.last_mut() {
            Some(last) => *last = transform,
            _ => (),
        }
        Ok(())
    }

    pub fn push_transform(
        &mut self,
        transform: mint::ColumnMatrix4<f32>,
    ) -> Result<(), RendererError> {
        self.flush()?;
        self.view_matrix_stack.push(transform);
        Ok(())
    }

    pub fn pop_transform(&mut self) -> Result<(), RendererError> {
        self.flush()?;
        self.view_matrix_stack.pop();
        Ok(())
    }

    /// Set the clear color and mark the frame buffer for clearing. The actual clear operation
    /// will be performed when the next batched render happens, or when `present` is called,
    /// whichever comes first.
    pub fn clear(&mut self, color: Color<f32>) -> Result<(), RendererError> {
        self.flush()?;
        self.pending_clear_color = Some(color.into());
        Ok(())
    }

    pub fn fill_rect(&mut self, rect: &Rect<f32>, color: [f32; 4]) -> Result<(), RendererError> {
        let pos_topleft = glam::Vec2::from(rect.location);
        let pos_topright = pos_topleft + glam::vec2(rect.dimensions.x, 0.0);
        let pos_bottomleft = pos_topleft + glam::vec2(0.0, rect.dimensions.y);
        let pos_bottomright = pos_bottomleft + glam::vec2(rect.dimensions.x, 0.0);

        let vertex_data = [
            Vertex::ptc(pos_topleft, [0.0, 0.0], &color),
            Vertex::ptc(pos_bottomleft, [0.0, 0.0], &color),
            Vertex::ptc(pos_bottomright, [0.0, 0.0], &color),
            Vertex::ptc(pos_topright, [0.0, 0.0], &color),
        ];
        let index_data: &[u16] = &[1, 2, 0, 2, 0, 3];

        self.stream_render(
            &StreamRenderArgs {
                texture: self.renderer.white_tex.clone(),
                shader: self.renderer.default_shader.clone(),
            },
            &vertex_data[..],
            index_data,
        )
    }

    fn clear_immediate(&mut self, color: Color<f32>) -> Result<(), RendererError> {
        self.flush()?;
        let _rpass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.frame.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: color.r as f64,
                        g: color.g as f64,
                        b: color.b as f64,
                        a: color.a as f64,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        Ok(())
    }

    pub fn stream_render(
        &mut self,
        args: &StreamRenderArgs,
        verts: &[Vertex],
        indices: &[u16],
    ) -> Result<(), RendererError> {
        if Some(args) != self.current_args.as_ref() {
            self.flush()?;
            self.current_args = Some(args.clone());
        }

        let indices: Vec<u16> = indices
            .iter()
            .map(|idx| idx + self.verts.len() as u16)
            .collect();

        self.verts.extend_from_slice(verts);
        self.indices.extend_from_slice(&indices[..]);

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), RendererError> {
        match &self.current_args {
            Some(args) => {
                let args = args.clone();
                self.do_flush(&args)
            }
            _ => Ok(()),
        }
    }

    pub fn present(mut self) -> Result<(), RendererError> {
        self.flush()?;
        if let Some(clear_color) = self.pending_clear_color {
            self.clear_immediate(clear_color.into())?;
        }

        let cmd = self.encoder.finish();
        self.renderer.queue.submit(Some(cmd));

        Ok(())
    }

    fn do_flush(&mut self, args: &StreamRenderArgs) -> Result<(), RendererError> {
        let device = &self.renderer.device;

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.verts),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.indices),
            usage: wgpu::BufferUsage::INDEX,
        });

        let view_matrix = self
            .view_matrix_stack
            .last()
            .map(|m| m.clone())
            .unwrap_or_else(|| glam::Mat4::identity().into());

        let bind_group = args.shader.bind_params(
            device,
            self.renderer.camera_size(),
            view_matrix,
            &args.texture,
        );

        let load_op = match self.pending_clear_color {
            Some(c) => wgpu::LoadOp::Clear(wgpu::Color {
                r: c[0] as f64,
                g: c[1] as f64,
                b: c[2] as f64,
                a: c[3] as f64,
            }),
            None => wgpu::LoadOp::Load,
        };
        self.pending_clear_color = None;

        // Scope render pass so that the render is encoded before the buffers are cleared.
        {
            let mut rpass =
                args.shader
                    .begin_render_pass(&self.frame.output.view, &mut self.encoder, load_op);

            rpass.set_bind_group(0, &bind_group, &[]);

            rpass.set_vertex_buffer(0, vertex_buf.slice(..));
            rpass.set_index_buffer(index_buf.slice(..));
            rpass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        }

        self.verts.clear();
        self.indices.clear();
        self.current_args = None;
        Ok(())
    }
}
