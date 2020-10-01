//! Traits and structs required to use custom WGPU devices.
//!
//! If using [`Renderer`] by itself through `riddle` there should be no need to use this
//! module.

use crate::{
    eventpub::EventSub,
    math::*,
    platform::{PlatformEvent, Window, WindowHandle},
    *,
};

use std::sync::Mutex;

/// A [`Renderer`] compatible WGPU device.
///
/// A default implementation exists for `riddle_platform_winit::Window`
/// in [`WindowWGPUDevice`].
///
/// The application may implement this trait to layer the renderer on
/// top of custom WGPU renderer.
///
/// # Example
///
/// ```no_run
/// use std::sync::Arc;
/// use riddle::{common::Color, platform::*, renderer::{ext::*, *}, *};
///
/// #[derive(Clone)]
/// struct ACustomRenderer {
///     // [..]
/// }
///
/// impl ACustomRenderer {
///     // [..]
/// #   fn new() -> Self { todo!() }
/// #   fn start_render(&self) { todo!() }
/// #   fn end_render(&self) { todo!() }
/// #   fn render_3d_scene(&self) { todo!() }
/// }
///
/// impl RendererWGPUDevice for ACustomRenderer {
///     // [..]
/// #   fn device(&self) -> &wgpu::Device { todo!() }
/// #   fn queue(&self) -> &wgpu::Queue { todo!() }
/// #   fn begin_frame(&self) -> Result<(), RendererError> { todo!() }
/// #   fn end_frame(&self) { todo!() }
/// #   fn viewport_dimensions(&self) -> math::Vector2<f32>  { todo!() }
/// #   fn with_frame(&self, f: &mut dyn FnMut(&wgpu::SwapChainFrame) -> Result<(), RendererError>) -> Result<(), RendererError>  { todo!() }
/// }
///
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///     let window = WindowBuilder::new().build(rdl.context())?;
///
///     let custom_renderer = ACustomRenderer::new();
///
///     let renderer = Renderer::new_from_device(Box::new(custom_renderer.clone()))?;
///
///     rdl.run(move |rdl| match rdl.event() {
///         Event::Platform(PlatformEvent::WindowClose(_)) => rdl.quit(),
///         Event::ProcessFrame => {
///             custom_renderer.start_render();
///             custom_renderer.render_3d_scene();
///
///             let mut render_ctx = renderer.begin_render().unwrap();
///             render_ctx.clear(Color::RED);
///             render_ctx.present();
///
///             custom_renderer.end_render();
///         }
///         _ => (),
///     })
/// }
/// ```
pub trait RendererWGPUDevice: Send + Sync {
    fn device(&self) -> &wgpu::Device;
    fn queue(&self) -> &wgpu::Queue;

    /// Called when the [`Renderer`] begins rendering to the swap chain frame.
    ///
    /// Invoked through [`Renderer::begin_render`]
    fn begin_frame(&self) -> Result<()>;

    /// When the renderer is done renderering to the swap chain frame.
    ///
    /// Invoked by a [`RenderContext::present`] call on the context returned from
    /// [`Renderer::begin_render`].
    fn end_frame(&self);

    /// The viewport dimensions of the swapchain frame.
    ///
    /// This controls the projection matrix used by the sprite renderer.
    fn viewport_dimensions(&self) -> Vector2<f32>;

    /// Provide a reference to the current swap chain frame for use by the
    /// renderer.
    fn with_frame(&self, f: &mut dyn FnMut(&wgpu::SwapChainFrame) -> Result<()>) -> Result<()>;
}

/// Extension trait for [`Renderer`] to support creating renderers on top
/// of custom WGPU devices.
///
/// See [`RendererWGPUDevice`] for more details about using custom WGPU devices.
pub trait RendererWGPU {
    /// The WGPU device which the renderer was constructed with
    fn wgpu_device(&self) -> &dyn RendererWGPUDevice;

    /// Create a [`Renderer`] from a custom device.
    fn new_from_device(device: Box<dyn RendererWGPUDevice>) -> Result<RendererHandle>;
}

/// A standard Renderer WGPU device used with `riddle_platform_winit` windows.
///
/// While this can be used directly, if using [`Renderer::new_from_window`], this
/// type shouldn't need to be used by consumer code.
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
            .ok_or(RendererError::APIInitError("Failed to get WGPU adapter"))?;

        let (device, queue) = futures::executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                shader_validation: true,
                ..Default::default()
            },
            None,
        ))
        .map_err(|_| RendererError::APIInitError("Failed to create WGPU device"))?;

        let (width, height) = window.physical_size();
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
            window: window.clone_handle(),
            window_event_sub,
            device,
            surface,
            queue,
            swap_chain: Mutex::new(swap_chain),
            current_frame: Mutex::new(None),
        })
    }

    fn handle_window_events(&self) {
        let mut dirty_swap_chain = false;
        for event in self.window_event_sub.collect().iter() {
            match event {
                PlatformEvent::WindowResize(_) => dirty_swap_chain = true,
                _ => (),
            }
        }

        if dirty_swap_chain {
            let (width, height) = self.window.physical_size();
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
    }

    fn ensure_current_frame(&self) -> Result<()> {
        let mut swap_chain = self.swap_chain.lock().unwrap();
        let mut frame = self.current_frame.lock().unwrap();

        let new_frame = swap_chain
            .get_current_frame()
            .map_err(|_| RendererError::BeginRenderError("Error getting swap chain frame"))?;

        *frame = Some(new_frame);

        Ok(())
    }

    fn present_current_frame(&self) -> () {
        let mut frame = self.current_frame.lock().unwrap();
        *frame = None;
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
        self.handle_window_events();
        self.ensure_current_frame()
    }

    fn end_frame(&self) {
        self.present_current_frame()
    }

    fn with_frame(&self, f: &mut dyn FnMut(&wgpu::SwapChainFrame) -> Result<()>) -> Result<()> {
        f(self.current_frame.lock().unwrap().as_ref().unwrap())
    }
}
