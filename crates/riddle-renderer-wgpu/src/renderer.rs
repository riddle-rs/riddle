use crate::{math::*, platform::*, *};

use riddle_common::eventpub::EventSub;
use riddle_macros::CloneHandle;

/// A simple 2D sprite based renderer.
///
/// A renderer is created for a Window and holds a reference to the window, which will keep
/// the window alive as long as the renderer is alive
#[derive(CloneHandle)]
pub struct Renderer {
    #[self_handle]
    weak_self: <Renderer as CloneHandle>::WeakHandle,

    pub(super) window: <Window as CloneHandle>::Handle,
    pub(super) default_shader: <Shader as CloneHandle>::Handle,
    pub(super) white_tex: <Texture as CloneHandle>::Handle,

    pub(super) device: wgpu::Device,
    surface: wgpu::Surface,
    pub(super) queue: wgpu::Queue,
    swap_chain: std::sync::Mutex<wgpu::SwapChain>,
    camera_size: std::sync::Mutex<Vector2<f32>>,

    window_event_sub: EventSub<PlatformEvent>,
}

impl Renderer {
    pub fn new_shared(window: &Window) -> Result<<Renderer as CloneHandle>::Handle, RendererError> {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

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
        white_img.set_pixel(0, 0, Color::from([0xFF; 4]));
        let white_tex = Texture::from_image(
            &device,
            &queue,
            white_img,
            FilterMode::Nearest,
            FilterMode::Nearest,
        )?
        .into();

        let camera_size = window.logical_size();

        let window_event_sub = EventSub::new();
        window.subscribe_to_events(&window_event_sub);

        let window_handle = window.clone_handle().ok_or(RendererError::Unknown)?;

        Ok(std::sync::Arc::new_cyclic(|weak_self| Self {
            weak_self: weak_self.clone(),
            window: window_handle,
            surface: surface,
            device,
            queue,
            swap_chain: std::sync::Mutex::new(swap_chain),
            default_shader: sprite_shader,
            white_tex,
            camera_size: std::sync::Mutex::new(Vector2 {
                x: camera_size.width as f32,
                y: camera_size.height as f32,
            }),
            window_event_sub,
        }))
    }

    pub fn begin_render_frame(&self) -> Result<FrameRenderer, RendererError> {
        self.handle_window_events()?;

        let frame = self
            .swap_chain
            .lock()
            .unwrap()
            .get_current_frame()
            .expect("Timeout when acquiring next swap chain texture");
        let encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Ok(FrameRenderer::new(self, frame, encoder))
    }

    pub fn camera_size(&self) -> Vector2<f32> {
        self.camera_size.lock().unwrap().clone()
    }

    pub fn window(&self) -> <Window as CloneHandle>::Handle {
        self.window.clone()
    }

    pub fn handle_window_events(&self) -> Result<(), RendererError> {
        let mut dirty_swap_chain = false;
        for event in self.window_event_sub.collect().iter() {
            match event {
                PlatformEvent::WindowResize(_) => dirty_swap_chain = true,
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
            *self.swap_chain.lock().unwrap() = swap_chain;
            let camera_size = self.window.logical_size();
            *self.camera_size.lock().unwrap() = Vector2 {
                x: camera_size.width as f32,
                y: camera_size.height as f32,
            };
        }

        Ok(())
    }
}
