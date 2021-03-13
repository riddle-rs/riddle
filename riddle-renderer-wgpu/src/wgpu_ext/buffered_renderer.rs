use crate::wgpu_ext::*;

use wgpu::util::DeviceExt;

#[derive(Clone)]
pub(crate) struct BufferedRenderArgs {
    pub(crate) texture: WGPUTextureHandle,
    pub(crate) shader: WGPUShaderHandle,
}

impl BufferedRenderArgs {
    fn new(desc: &WGPURenderableDesc) -> Self {
        Self {
            texture: desc.texture.clone(),
            shader: desc.shader.clone(),
        }
    }
}

impl PartialEq for BufferedRenderArgs {
    fn eq(&self, other: &Self) -> bool {
        WGPUTextureHandle::eq(&self.texture, &other.texture)
            && WGPUShaderHandle::eq(&self.shader, &other.shader)
    }
}

impl Eq for BufferedRenderArgs {}

pub(crate) struct BufferedRenderer<'a, Device, R>
where
    Device: WGPUDevice,
    R: WGPURenderTargetDesc<'a, Device>,
{
    target_desc: R,
    current_args: Option<BufferedRenderArgs>,

    verts: Vec<Vertex>,
    indices: Vec<u16>,

    pending_clear_color: Option<[f32; 4]>,
    view_matrix: mint::ColumnMatrix4<f32>,

    encoder: wgpu::CommandEncoder,
    _a_marker: std::marker::PhantomData<&'a Device>,
}

impl<'a, Device, R> BufferedRenderer<'a, Device, R>
where
    Device: WGPUDevice,
    R: WGPURenderTargetDesc<'a, Device>,
{
    pub fn new(target_desc: R, encoder: wgpu::CommandEncoder) -> Result<Self> {
        target_desc.begin_render()?;
        let identity: mint::ColumnMatrix4<f32> = glam::Mat4::identity().into();
        Ok(Self {
            target_desc,
            current_args: None,
            verts: vec![],
            indices: vec![],
            pending_clear_color: None,
            view_matrix: identity,
            encoder,
            _a_marker: std::marker::PhantomData::default(),
        })
    }

    fn clear_immediate(&mut self, color: Color<f32>) -> Result<()> {
        self.flush()?;
        let encoder = &mut self.encoder;
        self.target_desc.with_view(|view| {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: view,
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
                label: None,
            });
            Ok(())
        })
    }

    pub fn flush(&mut self) -> Result<()> {
        match &self.current_args {
            Some(args) => {
                let args = args.clone();
                self.do_flush(&args)
            }
            _ => Ok(()),
        }
    }

    fn do_flush(&mut self, args: &BufferedRenderArgs) -> Result<()> {
        let renderer = self.target_desc.renderer().clone_handle();

        let verts = std::mem::take(&mut self.verts);
        let indices = std::mem::take(&mut self.indices);
        let pending_clear = self.pending_clear_color.take();

        renderer.wgpu_device().with_device_info(|info| {
            let vertex_buf = info
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&verts),
                    usage: wgpu::BufferUsage::VERTEX,
                });

            let index_buf = info
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(&indices),
                    usage: wgpu::BufferUsage::INDEX,
                });

            let bind_group = args.shader.bind_params(
                info.device,
                self.target_desc.dimensions(),
                self.view_matrix,
                &args.texture,
            );

            let load_op = match pending_clear {
                Some(c) => wgpu::LoadOp::Clear(wgpu::Color {
                    r: c[0] as f64,
                    g: c[1] as f64,
                    b: c[2] as f64,
                    a: c[3] as f64,
                }),
                None => wgpu::LoadOp::Load,
            };

            // Scope render pass so that the render is encoded before the buffers are cleared.
            let encoder = &mut self.encoder;
            let indices_len = indices.len() as u32;
            self.target_desc.with_view(|view| {
                let mut rpass = args.shader.begin_render_pass(view, encoder, load_op);

                rpass.set_bind_group(0, &bind_group, &[]);

                rpass.set_vertex_buffer(0, vertex_buf.slice(..));
                rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..indices_len, 0, 0..1);
                Ok(())
            })?;

            self.verts.clear();
            self.indices.clear();
            self.current_args = None;
            Ok(())
        })
    }

    pub fn buffered_render(
        &mut self,
        args: &BufferedRenderArgs,
        verts: &[Vertex],
        indices: &[u16],
    ) -> Result<()> {
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
}

impl<'a, Device, R> RenderContext for BufferedRenderer<'a, Device, R>
where
    Device: WGPUDevice,
    R: WGPURenderTargetDesc<'a, Device>,
{
    fn set_transform(&mut self, transform: mint::ColumnMatrix4<f32>) -> Result<()> {
        self.flush()?;
        self.view_matrix = transform;
        Ok(())
    }

    /// Set the clear color and mark the frame buffer for clearing. The actual clear operation
    /// will be performed when the next batched render happens, or when `present` is called,
    /// whichever comes first.
    fn clear(&mut self, color: Color<f32>) -> Result<()> {
        self.flush()?;
        self.pending_clear_color = Some(color.into());
        Ok(())
    }

    fn fill_rect(&mut self, rect: &Rect<f32>, color: Color<f32>) -> Result<()> {
        let pos_topleft = glam::Vec2::from(rect.location);
        let pos_topright = pos_topleft + glam::vec2(rect.dimensions.x, 0.0);
        let pos_bottomleft = pos_topleft + glam::vec2(0.0, rect.dimensions.y);
        let pos_bottomright = pos_bottomleft + glam::vec2(rect.dimensions.x, 0.0);

        let color_arr: [f32; 4] = color.into();

        let vertex_data = [
            Vertex::ptc(pos_topleft, [0.0, 0.0], &color_arr),
            Vertex::ptc(pos_bottomleft, [0.0, 0.0], &color_arr),
            Vertex::ptc(pos_bottomright, [0.0, 0.0], &color_arr),
            Vertex::ptc(pos_topright, [0.0, 0.0], &color_arr),
        ];
        let index_data: &[u16] = &[1, 2, 0, 2, 0, 3];

        self.buffered_render(
            &BufferedRenderArgs {
                texture: self.target_desc.standard_resources().white_tex.clone(),
                shader: self.target_desc.standard_resources().default_shader.clone(),
            },
            &vertex_data[..],
            index_data,
        )
    }

    fn present(mut self) -> Result<()> {
        self.flush()?;
        if let Some(clear_color) = self.pending_clear_color {
            self.clear_immediate(clear_color.into())?;
        }

        let cmd = self.encoder.finish();
        self.target_desc
            .renderer()
            .wgpu_device()
            .with_device_info(|info| {
                info.queue.submit(Some(cmd));
                Ok(())
            })?;

        self.target_desc.end_render();

        Ok(())
    }

    fn render_internal<S: WGPURenderable>(&mut self, renderable: &S) -> Result<()> {
        renderable.with_renderable(|r| {
            self.buffered_render(&BufferedRenderArgs::new(r), r.verts, r.indices)?;
            Ok(())
        })
    }
}
