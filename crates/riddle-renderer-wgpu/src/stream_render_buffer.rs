use crate::*;

use std::rc::Rc;
use wgpu::util::DeviceExt;

#[derive(Clone)]
pub(crate) struct StreamRenderArgs {
    pub texture: Rc<Texture>,
    pub shader: Rc<Shader>,
}

impl PartialEq for StreamRenderArgs {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.texture, &other.texture) && Rc::ptr_eq(&self.shader, &other.shader)
    }
}

impl Eq for StreamRenderArgs {}

pub(crate) struct StreamRenderBuffer {
    current_args: Option<StreamRenderArgs>,

    verts: Vec<Vertex>,
    indices: Vec<u16>,
}

impl StreamRenderBuffer {
    pub fn new() -> Self {
        Self {
            current_args: None,
            verts: vec![],
            indices: vec![],
        }
    }

    pub fn stream_render(
        &mut self,
        renderer: &Renderer,
        args: &StreamRenderArgs,
        verts: &[Vertex],
        indices: &[u16],
    ) -> Result<(), RendererError> {
        if Some(args) != self.current_args.as_ref() {
            self.flush(renderer)?;
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

    pub fn flush(&mut self, renderer: &Renderer) -> Result<(), RendererError> {
        match &self.current_args {
            Some(args) => {
                let args = args.clone();
                self.do_flush(renderer, &args)
            }
            _ => Ok(()),
        }
    }

    fn do_flush(
        &mut self,
        renderer: &Renderer,
        args: &StreamRenderArgs,
    ) -> Result<(), RendererError> {
        let device = &renderer.device;
        let FrameRenderState { encoder, frame } = &mut *(renderer.get_frame_state()?);

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

        let bind_group = args
            .shader
            .bind_params(device, renderer.camera_size(), &args.texture);

        // Scope render pass so that the render is encoded before the buffers are cleared.
        {
            let mut rpass = args.shader.begin_render_pass(&frame.output.view, encoder);

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
