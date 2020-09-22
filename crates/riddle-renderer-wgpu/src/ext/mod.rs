use crate::{
    eventpub::EventSub,
    math::*,
    platform::{PlatformEvent, Window, WindowHandle},
    *,
};

use std::sync::Mutex;

pub trait RendererWGPUDevice: Send + Sync {
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;
    fn begin_frame(&self) -> Result<()>;
    fn end_frame(&self) -> Result<()>;
    fn viewport_dimensions(&self) -> Vector2<f32>;
    fn with_frame(&self, f: &mut dyn FnMut(&wgpu::SwapChainFrame) -> Result<()>) -> Result<()>;
}

pub trait RendererWGPU {
    fn wgpu_device(&self) -> &dyn RendererWGPUDevice;
}

pub struct WindowWGPUDevice {
    window: WindowHandle,
    window_event_sub: EventSub<PlatformEvent>,

    device: wgpu::Device,
    surface: wgpu::Surface,
    queue: wgpu::Queue,

    swap_chain: Mutex<wgpu::SwapChain>,
    current_frame: Mutex<Option<wgpu::SwapChainFrame>>,
}

impl WindowWGPUDevice {
    pub fn new(window: &Window) -> Result<Self> {
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

        let window_event_sub = EventSub::new();
        window.subscribe_to_events(&window_event_sub);

        Ok(Self {
            window: window.clone_handle().unwrap(),
            window_event_sub,
            device,
            surface,
            queue,
            swap_chain: Mutex::new(swap_chain),
            current_frame: Mutex::new(None),
        })
    }

    fn handle_window_events(&self) -> Result<()> {
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
        }

        Ok(())
    }

    fn ensure_current_frame(&self) -> Result<()> {
        let mut swap_chain = self.swap_chain.lock().unwrap();
        let mut frame = self.current_frame.lock().unwrap();

        let new_frame = swap_chain
            .get_current_frame()
            .map_err(|_| RendererError::Unknown)?;

        *frame = Some(new_frame);

        Ok(())
    }

    fn present_current_frame(&self) -> Result<()> {
        let mut frame = self.current_frame.lock().unwrap();
        *frame = None;
        Ok(())
    }
}

impl RendererWGPUDevice for WindowWGPUDevice {
    fn device(&self) -> &wgpu::Device {
        &self.device
    }

    fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    fn viewport_dimensions(&self) -> Vector2<f32> {
        self.window.logical_size().into()
    }

    fn begin_frame(&self) -> Result<()> {
        self.handle_window_events()?;
        self.ensure_current_frame()
    }

    fn end_frame(&self) -> Result<()> {
        self.present_current_frame()
    }

    fn with_frame(&self, f: &mut dyn FnMut(&wgpu::SwapChainFrame) -> Result<()>) -> Result<()> {
        f(self.current_frame.lock().unwrap().as_ref().unwrap())
    }
}
