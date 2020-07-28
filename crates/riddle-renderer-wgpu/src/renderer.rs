use crate::{math::*, window::*, *};

use riddle_common::eventpub::EventSub;

use std::{cell::RefCell, rc::Rc};

/// A simple 2D sprite based renderer.
///
/// A renderer is created for a Window and holds a reference to the window, which will keep
/// the window alive as long as the renderer is alive
pub struct Renderer {
    pub(super) window: Rc<Window>,
    pub(super) default_shader: Rc<Shader>,
    pub(super) white_tex: Rc<Texture>,
    pub(super) frame_state: RefCell<Option<FrameRenderState>>,

    pub(super) device: wgpu::Device,
    surface: wgpu::Surface,
    pub(super) queue: wgpu::Queue,
    swap_chain: RefCell<wgpu::SwapChain>,
    camera_size: RefCell<Vector2<f32>>,

    window_event_sub: EventSub<window::WindowEvent<Rc<Window>>>,

    pub(crate) stream_buffer: RefCell<StreamRenderBuffer>,
}

pub(super) struct FrameRenderState {
    pub encoder: wgpu::CommandEncoder,
    pub frame: wgpu::SwapChainFrame,
}

impl Renderer {
    pub fn new_shared(window: Rc<Window>) -> Result<Rc<Renderer>, RendererError> {
        Ok(Self::new(window)?.into())
    }

    fn new(window: Rc<Window>) -> Result<Renderer, RendererError> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window.as_ref()) };

        let adapter =
            futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            }))
            .ok_or(RendererError::Unknown)?;

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                shader_validation: true,
                ..Default::default()
            },
            None,
        ))
        .map_err(|_| RendererError::Unknown)?;

        let (width, height) = window.drawable_size();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let vs = include_bytes!("shaders/default.vert.spv");
        let fs = include_bytes!("shaders/default.frag.spv");
        let sprite_shader = Shader::from_readers(
            &device,
            std::io::Cursor::new(&vs[..]),
            std::io::Cursor::new(&fs[..]),
            wgpu::PrimitiveTopology::TriangleList,
        )?;

        let mut white_img = image::Image::new(1, 1).map_err(|_| RendererError::Unknown)?;
        white_img.set_pixel(0, 0, [0xFF; 4]);
        let white_tex = Texture::from_image(&device, &queue, white_img)?.into();

        let camera_size = window.logical_size();

        let window_event_sub = EventSub::new();
        window.subscribe_to_events(&window_event_sub);

        Ok(Self {
            window: window.clone(),
            surface: surface,
            device,
            queue,
            swap_chain: RefCell::new(swap_chain),
            frame_state: RefCell::new(None),
            default_shader: sprite_shader,
            white_tex,
            camera_size: RefCell::new(Vector2 {
                x: camera_size.width as f32,
                y: camera_size.height as f32,
            }),
            window_event_sub,
            stream_buffer: RefCell::new(StreamRenderBuffer::new()),
        })
    }

    pub(super) fn get_frame_state(
        &self,
    ) -> Result<std::cell::RefMut<FrameRenderState>, RendererError> {
        if self.frame_state.borrow().is_none() {
            self.handle_window_events()?;

            let frame = self
                .swap_chain
                .borrow_mut()
                .get_current_frame()
                .expect("Timeout when acquiring next swap chain texture");
            let encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            let mut fs = self
                .frame_state
                .try_borrow_mut()
                .map_err(|_| RendererError::Unknown)?;
            *fs = Some(FrameRenderState { encoder, frame });
        }

        Ok(std::cell::RefMut::map(self.frame_state.borrow_mut(), |s| {
            s.as_mut().unwrap()
        }))
    }

    /// Clear the output frame
    pub fn clear(&self) -> Result<(), RendererError> {
        self.stream_buffer.borrow_mut().flush(self)?;

        let FrameRenderState { encoder, frame, .. } = &mut *(self.get_frame_state()?);

        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.output.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        Ok(())
    }

    pub fn fill_rect(&self, rect: &Rect<f32>, color: [f32; 4]) -> Result<(), RendererError> {
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

        self.stream_buffer.borrow_mut().stream_render(
            self,
            &StreamRenderArgs {
                texture: self.white_tex.clone(),
                shader: self.default_shader.clone(),
            },
            &vertex_data[..],
            index_data,
        )
    }

    pub fn present(&self) -> Result<(), RendererError> {
        self.stream_buffer.borrow_mut().flush(self)?;

        let fs = self
            .frame_state
            .replace(None)
            .ok_or(RendererError::Unknown)?;
        self.queue.submit(Some(fs.encoder.finish()));
        Ok(())
    }

    pub fn camera_size(&self) -> Vector2<f32> {
        self.camera_size.borrow().clone()
    }

    pub fn window(&self) -> Rc<Window> {
        self.window.clone()
    }

    pub fn handle_window_events(&self) -> Result<(), RendererError> {
        let mut dirty_swap_chain = false;
        for event in self.window_event_sub.collect().iter() {
            match event {
                WindowEvent::WindowResize(_) => dirty_swap_chain = true,
                _ => (),
            }
        }

        if dirty_swap_chain {
            let (width, height) = self.window.drawable_size();
            let sc_desc = wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width,
                height,
                present_mode: wgpu::PresentMode::Mailbox,
            };

            let swap_chain = self.device.create_swap_chain(&self.surface, &sc_desc);
            *self.swap_chain.borrow_mut() = swap_chain;
            let camera_size = self.window.logical_size();
            *self.camera_size.borrow_mut() = Vector2 {
                x: camera_size.width as f32,
                y: camera_size.height as f32,
            };
        }

        Ok(())
    }
}
