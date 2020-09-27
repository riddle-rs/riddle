use crate::{ext::*, math::*, platform::*, *};

use riddle_common::define_handles;

/// A simple 2D sprite based renderer.
///
/// A renderer can be created for a Window and holds a reference to the window, which will
/// keep the window alive as long as the renderer is alive.
///
/// # Example
///
/// ```no_run
/// use riddle::{common::Color, platform::*, renderer::*, math::*, *};
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///     let window = WindowBuilder::new().build(rdl.context())?;
///
///     let renderer = Renderer::new_from_window(&window)?;
///
///     let mut render_ctx /*: impl RenderContext*/ = renderer.begin_render()?;
///     render_ctx.clear(Color::RED)?;
///
///     // Change the current transform matrix, and draw a rect
///     render_ctx.set_transform(glam::Mat4::from_scale(glam::vec3(2.0, 2.0, 1.0)).into())?;
///     render_ctx.fill_rect(&Rect::new(vec2(0.0, 0.0), vec2(10.0, 10.0)), Color::GREEN)?;
///
///     render_ctx.present()?;
///     Ok(())
/// }
/// ```
pub struct Renderer {
    weak_self: RendererWeak,

    wgpu_device: Box<dyn RendererWGPUDevice>,
    standard_res: StandardResources,
}

define_handles!(<Renderer>::weak_self, pub RendererHandle, pub RendererWeak);

impl Renderer {
    /// Initialize a new Renderer, creating a WGPU device for the window.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// let rdl =  RiddleLib::new()?;
    /// let window = WindowBuilder::new().build(rdl.context())?;
    ///
    /// let renderer = Renderer::new_from_window(&window)?;
    /// # Ok(()) }
    /// ```
    pub fn new_from_window(window: &Window) -> Result<RendererHandle> {
        let wgpu_device = WindowWGPUDevice::new(window)?;
        Self::new_from_device(Box::new(wgpu_device))
    }

    /// Get the frame dimensions as reported by the [`RendererWGPUDevice`].
    ///
    /// In the case of a default Window renderer, this will be the internal size of
    /// the window in logical units.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// let rdl =  RiddleLib::new()?;
    /// let window = WindowBuilder::new().dimensions(300, 400).build(rdl.context())?;
    ///
    /// let renderer = Renderer::new_from_window(&window)?;
    ///
    /// assert_eq!(vec2(300.0, 400.0), renderer.dimensions());
    /// # Ok(()) }
    /// ```
    pub fn dimensions(&self) -> Vector2<f32> {
        self.wgpu_device.viewport_dimensions()
    }

    /// Get a render context for the current swap chain frame.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle::{common::Color, platform::*, renderer::*, math::*, *};
    /// # fn main() -> Result<(), RiddleError> {
    /// # let rdl =  RiddleLib::new()?;
    /// # let window = WindowBuilder::new().build(rdl.context())?;
    /// let renderer = Renderer::new_from_window(&window)?;
    ///
    /// let mut render_ctx = renderer.begin_render()?;
    /// render_ctx.clear(Color::RED);
    /// render_ctx.present();
    /// # Ok(()) }
    /// ```
    pub fn begin_render<'a>(&'a self) -> Result<impl RenderContext + 'a> {
        let encoder = self
            .wgpu_device
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let target = SwapChainFrameTarget::new(self, self.dimensions());
        BufferedRenderer::new(target, encoder)
    }

    pub(crate) fn standard_res(&self) -> &StandardResources {
        &self.standard_res
    }
}

impl RendererWGPU for Renderer {
    fn wgpu_device(&self) -> &dyn RendererWGPUDevice {
        Box::as_ref(&self.wgpu_device)
    }

    /// Or the renderer can be built on top of existing WGPU contexts, to allow the simple
    /// renderer to be used on top of custom renderers.
    fn new_from_device(wgpu_device: Box<dyn RendererWGPUDevice>) -> Result<RendererHandle> {
        let vs = include_bytes!("shaders/default.vert.spv");
        let fs = include_bytes!("shaders/default.frag.spv");
        let sprite_shader = Shader::from_readers(
            wgpu_device.device(),
            std::io::Cursor::new(&vs[..]),
            std::io::Cursor::new(&fs[..]),
            wgpu::PrimitiveTopology::TriangleList,
        )?;

        let mut white_img = image::Image::new(1, 1);
        white_img.set_pixel(0, 0, Color::from([0xFF; 4]));
        let white_tex = Texture::from_image(
            wgpu_device.device(),
            wgpu_device.queue(),
            white_img,
            FilterMode::Nearest,
            FilterMode::Nearest,
            TextureType::Plain,
        )?
        .into();

        let standard_res = StandardResources {
            default_shader: sprite_shader,
            white_tex,
        };

        Ok(RendererHandle::new(|weak_self| Self {
            weak_self,
            wgpu_device,
            standard_res,
        }))
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct StandardResources {
    pub(super) default_shader: ShaderHandle,
    pub(super) white_tex: TextureHandle,
}
